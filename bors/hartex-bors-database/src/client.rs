/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::future::Future;
use std::pin::Pin;

use chrono::DateTime as ChronoDateTime;
use chrono::Utc;

use hartex_bors_core::models::BorsApproveBuild;
use hartex_bors_core::models::BorsBuild;
use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::models::BorsPullRequest;
use hartex_bors_core::models::BorsRepository;
use hartex_bors_core::models::BorsWorkflow;
use hartex_bors_core::models::BorsWorkflowStatus;
use hartex_bors_core::models::BorsWorkflowType;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::DatabaseClient;
use octocrab::models::pulls::PullRequest;
use octocrab::models::RunId;
use sea_orm::prelude::DateTime;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::TransactionTrait;

use crate::entity;

/// A SeaORM database client.
pub struct SeaORMDatabaseClient {
    connection: DatabaseConnection,
}

impl SeaORMDatabaseClient {
    /// Construct a new database client.
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl DatabaseClient for SeaORMDatabaseClient {
    fn associate_approve_build<'a>(
        &'a self,
        pr: &'a BorsPullRequest,
        branch: String,
        commit_hash: String,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let approve_build = entity::approve_build::ActiveModel {
                repository: Set(pr.repository.clone()),
                branch: Set(branch),
                commit_hash: Set(commit_hash),
                status: Set(build_status_to_database(BorsBuildStatus::Pending).to_string()),
                ..Default::default()
            };

            let tx = self.connection.begin().await?;
            let approve_build = entity::approve_build::Entity::insert(approve_build)
                .exec_with_returning(&tx)
                .await?;

            let pr_model = entity::pull_request::ActiveModel {
                id: Unchanged(pr.id),
                approve_build: Set(Some(approve_build.id)),
                ..Default::default()
            };
            pr_model.update(&tx).await?;
            tx.commit().await?;

            Ok(())
        })
    }

    fn associate_try_build<'a>(
        &'a self,
        pr: &'a BorsPullRequest,
        branch: String,
        commit_hash: String,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let build = entity::build::ActiveModel {
                repository: Set(pr.repository.clone()),
                branch: Set(branch),
                commit_hash: Set(commit_hash),
                status: Set(build_status_to_database(BorsBuildStatus::Pending).to_string()),
                ..Default::default()
            };

            let tx = self.connection.begin().await?;
            let build = entity::build::Entity::insert(build)
                .exec_with_returning(&tx)
                .await?;

            let pr_model = entity::pull_request::ActiveModel {
                id: Unchanged(pr.id),
                try_build: Set(Some(build.id)),
                ..Default::default()
            };
            pr_model.update(&tx).await?;
            tx.commit().await?;

            Ok(())
        })
    }

    fn create_repository<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let repo = entity::repository::ActiveModel {
                repository: Set(format!("{name}")),
                ..Default::default()
            };

            match entity::repository::Entity::insert(repo)
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&self.connection)
                .await
            {
                Ok(_) | Err(DbErr::RecordNotInserted) => Ok(()),
                Err(error) => return Err(error.into()),
            }
        })
    }

    fn create_workflow_with_approve_build<'a>(
        &'a self,
        approve_build: &'a BorsApproveBuild,
        name: String,
        url: String,
        run_id: RunId,
        workflow_type: BorsWorkflowType,
        workflow_status: BorsWorkflowStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let workflow = entity::workflow::ActiveModel {
                approve_build: Set(Some(approve_build.id)),
                name: Set(name),
                url: Set(url),
                run_id: Set(run_id.0 as i64),
                r#type: Set(workflow_type_to_database(workflow_type).to_string()),
                status: Set(workflow_status_to_database(workflow_status).to_string()),
                ..Default::default()
            };

            workflow.insert(&self.connection).await?;

            Ok(())
        })
    }

    fn create_workflow_with_try_build<'a>(
        &'a self,
        build: &'a BorsBuild,
        name: String,
        url: String,
        run_id: RunId,
        workflow_type: BorsWorkflowType,
        workflow_status: BorsWorkflowStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let workflow = entity::workflow::ActiveModel {
                build: Set(Some(build.id)),
                name: Set(name),
                url: Set(url),
                run_id: Set(run_id.0 as i64),
                r#type: Set(workflow_type_to_database(workflow_type).to_string()),
                status: Set(workflow_status_to_database(workflow_status).to_string()),
                ..Default::default()
            };

            workflow.insert(&self.connection).await?;

            Ok(())
        })
    }

    fn find_approve_build<'a>(
        &'a self,
        repository: &'a GithubRepositoryName,
        branch: String,
        commit_sha: String,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Option<BorsApproveBuild>>> + '_>> {
        Box::pin(async move {
            let approve_build = entity::approve_build::Entity::find()
                .filter(
                    entity::approve_build::Column::Repository
                        .eq(&format!(
                            "{}/{}",
                            repository.owner(),
                            repository.repository()
                        ))
                        .and(entity::approve_build::Column::Branch.eq(branch))
                        .and(entity::approve_build::Column::CommitHash.eq(commit_sha)),
                )
                .one(&self.connection)
                .await?;

            Ok(approve_build.map(approve_build_from_database))
        })
    }

    fn find_build<'a>(
        &'a self,
        repository: &'a GithubRepositoryName,
        branch: String,
        commit_sha: String,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Option<BorsBuild>>> + '_>> {
        Box::pin(async move {
            let build = entity::build::Entity::find()
                .filter(
                    entity::build::Column::Repository
                        .eq(&format!(
                            "{}/{}",
                            repository.owner(),
                            repository.repository()
                        ))
                        .and(entity::build::Column::Branch.eq(branch))
                        .and(entity::build::Column::CommitHash.eq(commit_sha)),
                )
                .one(&self.connection)
                .await?;

            Ok(build.map(build_from_database))
        })
    }

    fn find_pull_request_by_approve_build<'a>(
        &'a self,
        approve_build: &'a BorsApproveBuild,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Option<BorsPullRequest>>> + '_>> {
        Box::pin(async move {
            let result = crate::select_pr::SelectPullRequest::exec_with_approve_build_one(
                &self.connection,
                approve_build,
            )
            .await?;

            Ok(result.map(|(pr, approve_build, build)| pr_from_database(pr, approve_build, build)))
        })
    }

    fn find_pull_request_by_try_build<'a>(
        &'a self,
        build: &'a BorsBuild,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Option<BorsPullRequest>>> + '_>> {
        Box::pin(async move {
            let result = crate::select_pr::SelectPullRequest::exec_with_try_build_one(
                &self.connection,
                build,
            )
            .await?;

            Ok(result.map(|(pr, approve_build, build)| pr_from_database(pr, approve_build, build)))
        })
    }

    fn get_or_create_pull_request<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
        approved_by: Option<String>,
        github_pr: &'a PullRequest,
        pr_number: u64,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<BorsPullRequest>> + '_>> {
        Box::pin(async move {
            let pr = entity::pull_request::ActiveModel {
                repository: Set(format!("{name}")),
                number: Set(pr_number as i32),
                assignee: Set(github_pr
                    .assignees
                    .as_ref()
                    .and_then(|authors| authors.first())
                    .and_then(|author| Some(author.login.clone()))
                    .unwrap_or_default()),
                approved_by: Set(approved_by),
                title: Set(github_pr.title.clone().unwrap()),
                head_ref: Set(github_pr
                    .head
                    .label
                    .clone()
                    .unwrap_or(String::from("<unknown>"))),
                url: Set(format!("https://github.com/{name}/pull/{pr_number}")),
                ..Default::default()
            };

            match entity::pull_request::Entity::insert(pr)
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&self.connection)
                .await
            {
                Ok(_) => {}
                Err(DbErr::RecordNotInserted) => {
                    // the record is already in the database
                }
                Err(error) => return Err(error.into()),
            }

            let (pr, approve_build, build) =
                crate::select_pr::SelectPullRequest::exec_with_repo_one(
                    &self.connection,
                    format!("{name}"),
                )
                .await?
                .unwrap();

            Ok(pr_from_database(pr, approve_build, build))
        })
    }

    fn get_pull_requests_for_repository<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsPullRequest>>> + Send + '_>> {
        Box::pin(async move {
            let pull_requests = crate::select_pr::SelectPullRequest::exec_with_repo_many(
                &self.connection,
                format!("{name}"),
            )
            .await?;

            Ok(pull_requests
                .into_iter()
                .map(|(pull_request, approve_build, build)| {
                    pr_from_database(pull_request, approve_build, build)
                })
                .collect())
        })
    }

    fn get_repositories(
        &self,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsRepository>>> + Send + '_>> {
        Box::pin(async move {
            let repositories = entity::repository::Entity::find()
                .all(&self.connection)
                .await?;

            Ok(repositories
                .into_iter()
                .map(|repository| repository_from_database(repository))
                .collect())
        })
    }

    fn get_workflows_for_approve_build<'a>(
        &'a mut self,
        approve_build: &'a BorsApproveBuild,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsWorkflow>>> + '_>> {
        Box::pin(async move {
            let workflows = crate::select_workflow::SelectWorkflow::exec_with_approve_build_many(
                &self.connection,
                approve_build,
            )
            .await?;

            Ok(workflows
                .into_iter()
                .map(|(workflow, approve_build, build)| {
                    workflow_from_database(workflow, approve_build, build)
                })
                .collect())
        })
    }

    fn get_workflows_for_try_build<'a>(
        &'a mut self,
        build: &'a BorsBuild,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsWorkflow>>> + '_>> {
        Box::pin(async move {
            let workflows = crate::select_workflow::SelectWorkflow::exec_with_try_build_many(
                &self.connection,
                build,
            )
            .await?;

            Ok(workflows
                .into_iter()
                .map(|(workflow, approve_build, build)| {
                    workflow_from_database(workflow, approve_build, build)
                })
                .collect())
        })
    }

    fn update_approve_build_status<'a>(
        &'a self,
        approve_build: &'a BorsApproveBuild,
        status: BorsBuildStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let model = entity::approve_build::ActiveModel {
                id: Unchanged(approve_build.id),
                status: Set(build_status_to_database(status).to_string()),
                ..Default::default()
            };
    
            model.update(&self.connection).await?;
    
            Ok(())
        })
    }

    fn update_build_status<'a>(
        &'a self,
        build: &'a BorsBuild,
        status: BorsBuildStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let model = entity::build::ActiveModel {
                id: Unchanged(build.id),
                status: Set(build_status_to_database(status).to_string()),
                ..Default::default()
            };

            model.update(&self.connection).await?;

            Ok(())
        })
    }

    fn update_workflow_status(
        &self,
        run_id: u64,
        status: BorsWorkflowStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>> {
        Box::pin(async move {
            let workflow = entity::workflow::ActiveModel {
                status: Set(workflow_status_to_database(status).to_string()),
                ..Default::default()
            };

            entity::workflow::Entity::update_many()
                .set(workflow)
                .filter(entity::workflow::Column::RunId.eq(run_id))
                .exec(&self.connection)
                .await?;

            Ok(())
        })
    }
}

fn approve_build_from_database(model: entity::approve_build::Model) -> BorsApproveBuild {
    BorsApproveBuild {
        id: model.id,
        repository: model.repository,
        branch: model.branch,
        commit_hash: model.commit_hash,
        status: build_status_from_database(model.status),
        created_at: datetime_from_database(model.created_at),
    }
}

fn build_from_database(model: entity::build::Model) -> BorsBuild {
    BorsBuild {
        id: model.id,
        repository: model.repository,
        branch: model.branch,
        commit_hash: model.commit_hash,
        status: build_status_from_database(model.status),
        created_at: datetime_from_database(model.created_at),
    }
}

fn build_status_to_database(status: BorsBuildStatus) -> &'static str {
    match status {
        BorsBuildStatus::Pending => "pending",
        BorsBuildStatus::Success => "success",
        BorsBuildStatus::Failure => "failure",
        BorsBuildStatus::Cancelled => "cancelled",
    }
}

fn build_status_from_database(status: String) -> BorsBuildStatus {
    match status.as_str() {
        "pending" => BorsBuildStatus::Pending,
        "success" => BorsBuildStatus::Success,
        "failure" => BorsBuildStatus::Failure,
        "cancelled" => BorsBuildStatus::Cancelled,
        _ => unreachable!(),
    }
}

fn datetime_from_database(datetime: DateTime) -> DateTimeUtc {
    ChronoDateTime::from_utc(datetime, Utc)
}

fn pr_from_database(
    pr: entity::pull_request::Model,
    approve_build: Option<entity::approve_build::Model>,
    build: Option<entity::build::Model>,
) -> BorsPullRequest {
    BorsPullRequest {
        id: pr.id,
        repository: pr.repository,
        number: pr.number as u64,
        assignee: pr.assignee,
        approved_by: pr.approved_by,
        title: pr.title,
        head_ref: pr.head_ref,
        approve_build: approve_build.map(approve_build_from_database),
        try_build: build.map(build_from_database),
        url: pr.url,
        created_at: datetime_from_database(pr.created_at),
    }
}

fn repository_from_database(repository: entity::repository::Model) -> BorsRepository {
    BorsRepository {
        name: repository.repository,
    }
}

// FIXME: workflows are either a try build or an approve build.
fn workflow_from_database(
    workflow: entity::workflow::Model,
    approve_build: Option<entity::approve_build::Model>,
    build: Option<entity::build::Model>,
) -> BorsWorkflow {
    BorsWorkflow {
        id: workflow.id,
        approve_build: approve_build.map(approve_build_from_database),
        build: build.map(build_from_database),
        name: workflow.name,
        url: workflow.url,
        run_id: RunId(workflow.run_id as u64),
        workflow_type: workflow_type_from_database(&workflow.r#type),
        workflow_status: workflow_status_from_database(&workflow.status),
        created_at: datetime_from_database(workflow.created_at),
    }
}

fn workflow_status_from_database(workflow_status: &str) -> BorsWorkflowStatus {
    match workflow_status {
        "pending" => BorsWorkflowStatus::Pending,
        "failure" => BorsWorkflowStatus::Failure,
        "success" => BorsWorkflowStatus::Success,
        _ => BorsWorkflowStatus::Pending,
    }
}

fn workflow_status_to_database(workflow_status: BorsWorkflowStatus) -> &'static str {
    match workflow_status {
        BorsWorkflowStatus::Pending => "pending",
        BorsWorkflowStatus::Failure => "failure",
        BorsWorkflowStatus::Success => "success",
    }
}

fn workflow_type_from_database(workflow_type: &str) -> BorsWorkflowType {
    match workflow_type {
        "external" => BorsWorkflowType::External,
        "github" => BorsWorkflowType::GitHub,
        _ => unreachable!(),
    }
}

fn workflow_type_to_database(workflow_type: BorsWorkflowType) -> &'static str {
    match workflow_type {
        BorsWorkflowType::External => "external",
        BorsWorkflowType::GitHub => "github",
    }
}
