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

//! # Approve command
//!
//! bors r+

use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::models::Permission;
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use hartex_bors_github::messages::auto_merge_commit_message;

use crate::permissions::check_permissions;

pub const APPROVE_BRANCH_NAME: &str = "automation/bors/approve";
const APPROVE_MERGE_BRANCH_NAME: &str = "automation/bors/approve-merge";

/// Executes the approve command.
pub async fn approve_command<C: RepositoryClient>(
    repository: &mut GithubRepositoryState<C>,
    database: &mut dyn DatabaseClient,
    pr: u64,
    author: &str,
) -> hartex_eyre::Result<()> {
    if !check_permissions(repository, pr, author, Permission::Approve).await? {
        return Ok(());
    }

    let github_pr = repository.client.get_pull_request(pr).await?;

    repository
        .client
        .post_comment(
            pr,
            &format!(
                ":pushpin: Commit {} has been approved by `{author}`.",
                github_pr.head.sha
            ),
        )
        .await?;

    let pr_model = database
        .get_or_create_pull_request(
            repository.client.repository_name(),
            Some(author.to_string()),
            &github_pr,
            pr,
        )
        .await?;

    if let Some(ref build) = pr_model.approve_build && build.status == BorsBuildStatus::Pending {
        repository
            .client
            .post_comment(pr, ":warning: A build is currently in progress. You can cancel the build using `bors r-`")
            .await?;

        return Ok(());
    };

    repository
        .client
        .set_branch_to_revision(APPROVE_MERGE_BRANCH_NAME, &github_pr.base.sha)
        .await?;

    let merge_hash = repository
        .client
        .merge_branches(
            APPROVE_MERGE_BRANCH_NAME,
            &github_pr.head.sha,
            &auto_merge_commit_message(&github_pr, author),
        )
        .await?;

    database
        .associate_approve_build(&pr_model, APPROVE_BRANCH_NAME.to_string(), merge_hash.clone())
        .await?;

    
    repository
        .client
        .set_branch_to_revision(APPROVE_BRANCH_NAME, &merge_hash)
        .await?;

    
    repository
        .client
        .delete_branch(APPROVE_MERGE_BRANCH_NAME)
        .await?;

    Ok(())
}
