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
use hartex_bors_core::DatabaseClient;
use hartex_log::log;
use octocrab::models::workflows::Run;
use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::models::BorsWorkflowStatus;
use hartex_bors_core::models::BorsWorkflowType;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_github::GithubRepositoryClient;

struct CheckSuiteCompleted {
    repository: GithubRepositoryName,
    branch: String,
    commit_hash: String,
}

pub(crate) async fn workflow_completed(
    repository: &GithubRepositoryState<GithubRepositoryClient>,
    database: &mut dyn DatabaseClient,
    run: Run,
) -> hartex_eyre::Result<()> {
    todo!()
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
    database.create_workflow(
        &build,
        run.name,
        run.url.to_string(),
        run.id,
        BorsWorkflowType::GitHub,
        BorsWorkflowStatus::Pending,
    ).await?;

    Ok(())
}

fn is_relevant_branch(branch: &str) -> bool {
    [TRY_BRANCH_NAME].contains(&branch)
}
