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
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use hartex_bors_github::GithubRepositoryClient;
use hartex_log::log;
use octocrab::models::workflows::Run;

struct CheckSuiteCompleted {
    repository: GithubRepositoryName,
    branch: String,
    commit_hash: String,
}

pub(crate) async fn workflow_completed(
    repository: &mut GithubRepositoryState<GithubRepositoryClient>,
    database: &mut dyn DatabaseClient,
    run: Run,
) -> hartex_eyre::Result<()> {
    log::trace!(
        r#"updating status of workflow of {} to "{}""#,
        run.url,
        run.status
    );
    database
        .update_workflow_status(
            run.id.0 as u64,
            string_to_workflow_status(run.conclusion.unwrap_or_default().as_str()),
        )
        .await?;

    if run.head_branch.contains("try") {
        complete_try_build(
            repository,
            database,
            CheckSuiteCompleted {
                repository: GithubRepositoryName::new_from_repository(run.repository)?,
                branch: run.head_branch,
                commit_hash: run.head_sha,
            },
        )
        .await?;
    }

    Ok(())
}

pub(crate) async fn workflow_started(
    repository: &GithubRepositoryState<GithubRepositoryClient>,
    database: &mut dyn DatabaseClient,
    run: Run,
) -> hartex_eyre::Result<()> {
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

    let Some(build) = database.find_build(&repository.repository, run.head_branch.clone(), run.head_sha.clone()).await? else {
        return Ok(());
    };

    if build.status != BorsBuildStatus::Pending {
        return Ok(());
    }

    log::trace!("creating workflow in database");
    database
        .create_workflow(
            &build,
            run.name,
            run.url.to_string(),
            run.id,
            BorsWorkflowType::GitHub,
            BorsWorkflowStatus::Pending,
        )
        .await?;

    Ok(())
}

async fn complete_try_build(
    repository: &mut GithubRepositoryState<GithubRepositoryClient>,
    database: &mut dyn DatabaseClient,
    event: CheckSuiteCompleted,
) -> hartex_eyre::Result<()> {
    if !is_relevant_branch(&event.branch) {
        return Ok(());
    }

    let Some(build) = database
        .find_build(
            &event.repository,
            event.branch.clone(),
            event.commit_hash.clone(),
        ).await? else {
        log::warn!("received workflow completed for nonexistent workflow...?");

        return Ok(());
    };

    if build.status != BorsBuildStatus::Pending {
        return Ok(());
    }

    let Some(pull_request) = database.find_pull_request_by_try_build(&build).await? else {
        log::warn!("no pull request is found for the build {}", build.commit_hash);

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

    let mut workflows = database.get_workflows_for_build(&build).await?;
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

    let message = if !has_failure {
        log::info!("Workflow succeeded");

        let hash = &event.commit_hash;
        format!(
            r#":sunny: Try build successful
{workflow_list}

Build commit: {hash} (`{hash}`)"#
        )
    } else {
        log::info!("Workflow failed");
        format!(
            r#":broken_heart: Try build failed
{workflow_list}"#
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
    [TRY_BRANCH_NAME].contains(&branch)
}

fn string_to_workflow_status(string: &str) -> BorsWorkflowStatus {
    match string {
        "success" => BorsWorkflowStatus::Success,
        _ => BorsWorkflowStatus::Failure,
    }
}
