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

use hartex_bors_commands::commands::r#try::TRY_BRANCH_NAME;
use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::models::BorsWorkflowStatus;
use hartex_bors_core::models::BorsWorkflowType;
use hartex_bors_core::models::CheckStatus;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::queue::BorsQueueEvent;
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use hartex_bors_github::GithubRepositoryClient;
use hartex_log::log;
use miette::IntoDiagnostic;
use octocrab::models::workflows::Run;
use tokio::sync::mpsc::Sender;

use crate::queue::APPROVE_BRANCH_NAME;

struct CheckSuiteCompleted {
    repository: GithubRepositoryName,
    branch: String,
    commit_hash: String,
}

pub(crate) async fn workflow_completed(
    repository: &GithubRepositoryState<GithubRepositoryClient>,
    database: &dyn DatabaseClient,
    run: Run,
    sender: Sender<BorsQueueEvent>,
) -> miette::Result<()> {
    log::trace!(
        r#"updating status of workflow of {} to "{}""#,
        run.url,
        run.status
    );
    database
        .update_workflow_status(
            run.id.0,
            string_to_workflow_status(run.conclusion.unwrap_or_default().as_str()),
        )
        .await?;

    if run.head_branch.clone().contains("try") {
        complete_try_build(
            repository,
            database,
            CheckSuiteCompleted {
                repository: GithubRepositoryName::new_from_repository(run.repository.clone())?,
                branch: run.head_branch.clone(),
                commit_hash: run.head_sha.clone(),
            },
        )
        .await?;
    }

    if run.head_branch.contains("approve") {
        complete_approve_build(
            repository,
            database,
            CheckSuiteCompleted {
                repository: GithubRepositoryName::new_from_repository(run.repository)?,
                branch: run.head_branch,
                commit_hash: run.head_sha,
            },
            sender,
        )
        .await?;
    }

    Ok(())
}

pub(crate) async fn workflow_started(
    repository: &GithubRepositoryState<GithubRepositoryClient>,
    database: &dyn DatabaseClient,
    run: Run,
) -> miette::Result<()> {
    if !is_relevant_branch(&run.head_branch) {
        return Ok(());
    }

    log::trace!(
        "handling workflow started (name={}, url={}, branch={}, commit={})",
        run.name,
        run.url,
        run.head_branch,
        run.head_sha,
    );

    if run.head_branch.contains("try") {
        let Some(build) = database
            .find_build(
                &repository.repository,
                run.head_branch.clone(),
                run.head_sha.clone(),
            )
            .await?
        else {
            return Ok(());
        };

        if build.status != BorsBuildStatus::Pending {
            return Ok(());
        }

        log::trace!("creating workflow in database");
        database
            .create_workflow_with_try_build(
                &build,
                run.name.clone(),
                run.url.to_string(),
                run.id,
                BorsWorkflowType::GitHub,
                BorsWorkflowStatus::Pending,
            )
            .await?;
    }

    if run.head_branch.contains("approve") {
        let Some(approve_build) = database
            .find_approve_build(
                &repository.repository,
                run.head_branch.clone(),
                run.head_sha.clone(),
            )
            .await?
        else {
            return Ok(());
        };

        if approve_build.status != BorsBuildStatus::Pending {
            return Ok(());
        }

        log::trace!("creating workflow in database");
        database
            .create_workflow_with_approve_build(
                &approve_build,
                run.name,
                run.url.to_string(),
                run.id,
                BorsWorkflowType::GitHub,
                BorsWorkflowStatus::Pending,
            )
            .await?;
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
async fn complete_approve_build(
    repository: &GithubRepositoryState<GithubRepositoryClient>,
    database: &dyn DatabaseClient,
    event: CheckSuiteCompleted,
    sender: Sender<BorsQueueEvent>,
) -> miette::Result<()> {
    if !is_relevant_branch(&event.branch) {
        return Ok(());
    }

    let Some(approve_build) = database
        .find_approve_build(
            &event.repository,
            event.branch.clone(),
            event.commit_hash.clone(),
        )
        .await?
    else {
        log::warn!("received workflow completed for nonexistent workflow...?");

        return Ok(());
    };

    if approve_build.status != BorsBuildStatus::Pending {
        return Ok(());
    }

    let Some(pull_request) = database
        .find_pull_request_by_approve_build(&approve_build)
        .await?
    else {
        log::warn!(
            "no pull request is found for the build {}",
            approve_build.commit_hash
        );

        return Ok(());
    };

    let checks = repository
        .client
        .get_checks_for_commit(&event.branch, &event.commit_hash)
        .await?;

    if checks
        .iter()
        .any(|check| matches!(check.status, CheckStatus::Pending))
    {
        return Ok(());
    }

    let has_failure = checks
        .iter()
        .any(|check| matches!(check.status, CheckStatus::Failure));

    if has_failure {
        let mut workflows = database
            .get_workflows_for_approve_build(&approve_build)
            .await?;
        workflows.sort_by(|a, b| a.name.cmp(&b.name));

        let workflow_list = workflows
            .into_iter()
            .map(|w| {
                format!(
                    "- [{}]({}) {}",
                    w.name,
                    w.url,
                    if w.workflow_status == BorsWorkflowStatus::Success {
                        ":white_check_mark:"
                    } else {
                        ":x:"
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        repository
            .client
            .post_comment(
                pull_request.number,
                &format!(
                    r#":broken_heart: Build failed
{workflow_list}"#
                ),
            )
            .await?;
    } else {
        let github_pr = repository
            .client
            .get_pull_request(pull_request.number)
            .await?;

        repository
            .client
            .post_comment(
                pull_request.number,
                &format!(
                    r#":sunny: Build successful
Approved by: {}
Pushing {} to {}..."#,
                    pull_request
                        .approved_by
                        .unwrap_or(String::from("<unknown>")),
                    &github_pr.head.sha,
                    &github_pr.base.ref_field
                ),
            )
            .await?;

        let revision = repository.client.get_revision(APPROVE_BRANCH_NAME).await?;

        repository
            .client
            .set_branch_to_revision(&github_pr.base.ref_field, &revision)
            .await?;
    }

    let status = if has_failure {
        BorsBuildStatus::Failure
    } else {
        BorsBuildStatus::Success
    };

    database
        .update_approve_build_status(&approve_build, status)
        .await?;

    if has_failure {
        sender
            .send(BorsQueueEvent::PullRequestFailed(
                repository.repository.clone(),
                pull_request.id,
            ))
            .await
            .into_diagnostic()?;
    } else {
        sender
            .send(BorsQueueEvent::PullRequestMerged(
                repository.repository.clone(),
                pull_request.id,
            ))
            .await
            .into_diagnostic()?;
    }

    Ok(())
}

async fn complete_try_build(
    repository: &GithubRepositoryState<GithubRepositoryClient>,
    database: &dyn DatabaseClient,
    event: CheckSuiteCompleted,
) -> miette::Result<()> {
    if !is_relevant_branch(&event.branch) {
        return Ok(());
    }

    let Some(build) = database
        .find_build(
            &event.repository,
            event.branch.clone(),
            event.commit_hash.clone(),
        )
        .await?
    else {
        log::warn!("received workflow completed for nonexistent workflow...?");

        return Ok(());
    };

    if build.status != BorsBuildStatus::Pending {
        return Ok(());
    }

    let Some(pull_request) = database.find_pull_request_by_try_build(&build).await? else {
        log::warn!(
            "no pull request is found for the build {}",
            build.commit_hash
        );

        return Ok(());
    };

    let checks = repository
        .client
        .get_checks_for_commit(&event.branch, &event.commit_hash)
        .await?;

    if checks
        .iter()
        .any(|check| matches!(check.status, CheckStatus::Pending))
    {
        return Ok(());
    }

    let has_failure = checks
        .iter()
        .any(|check| matches!(check.status, CheckStatus::Failure));

    let mut workflows = database.get_workflows_for_try_build(&build).await?;
    workflows.sort_by(|a, b| a.name.cmp(&b.name));

    let workflow_list = workflows
        .into_iter()
        .map(|w| {
            format!(
                "- [{}]({}) {}",
                w.name,
                w.url,
                if w.workflow_status == BorsWorkflowStatus::Success {
                    ":white_check_mark:"
                } else {
                    ":x:"
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let message = if has_failure {
        log::info!("Workflow failed");
        format!(
            r#":broken_heart: Try build failed
{workflow_list}"#
        )
    } else {
        log::info!("Workflow succeeded");

        let hash = &event.commit_hash;
        format!(
            r#":sunny: Try build successful
{workflow_list}

Build commit: {hash} (`{hash}`)"#
        )
    };
    repository
        .client
        .post_comment(pull_request.number, &message)
        .await?;

    let status = if has_failure {
        BorsBuildStatus::Failure
    } else {
        BorsBuildStatus::Success
    };

    database.update_build_status(&build, status).await?;

    Ok(())
}

fn is_relevant_branch(branch: &str) -> bool {
    [APPROVE_BRANCH_NAME, TRY_BRANCH_NAME].contains(&branch)
}

fn string_to_workflow_status(string: &str) -> BorsWorkflowStatus {
    match string {
        "success" => BorsWorkflowStatus::Success,
        _ => BorsWorkflowStatus::Failure,
    }
}
