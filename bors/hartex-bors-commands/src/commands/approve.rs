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
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::models::Permission;
use hartex_bors_core::queue::BorsQueueEvent;
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use miette::IntoDiagnostic;
use tokio::sync::mpsc::Sender;

use crate::permissions::check_permissions;

/// Executes the approve command.
pub async fn approve_command<C: RepositoryClient>(
    repository: &GithubRepositoryState<C>,
    database: &dyn DatabaseClient,
    pr: u64,
    approver: &str,
    sender: Sender<BorsQueueEvent>,
) -> miette::Result<()> {
    if !check_permissions(repository, pr, approver, Permission::Approve).await? {
        return Ok(());
    }

    let github_pr = repository.client.get_pull_request(pr).await?;

    repository
        .client
        .post_comment(
            pr,
            &format!(
                ":pushpin: Commit {} has been approved by `{approver}`.",
                github_pr.head.sha
            ),
        )
        .await?;

    let mut labels = github_pr.clone().labels.unwrap();
    labels.retain(|label| label.name != "waiting-on-review");

    let label = repository.client.get_label("waiting-on-bors").await?;
    labels.push(label);

    repository
        .client
        .set_labels_of_pull_request(labels.iter().map(|label| label.name.clone()).collect(), pr)
        .await?;

    let pr_model = database
        .get_or_create_pull_request(repository.client.repository_name(), &github_pr, pr)
        .await?;

    database.approve_pull_request(&pr_model, approver).await?;

    if let Some(ref build) = pr_model.approve_build && build.status == BorsBuildStatus::Pending {
        repository
            .client
            .post_comment(pr, ":warning: A build is currently in progress. You can cancel the build using `bors r-`")
            .await?;

        return Ok(());
    };

    database
        .enqueue_pull_request(&repository.repository, &pr_model)
        .await?;
    sender
        .send(BorsQueueEvent::PullRequestEnqueued(
            GithubRepositoryName::new_from_string(pr_model.repository)?,
            pr_model.id,
        ))
        .await
        .into_diagnostic()?;

    Ok(())
}
