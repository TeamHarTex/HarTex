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

use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::models::Permission;
use hartex_bors_core::RepositoryClient;
use hartex_bors_database::models::BorsBuildStatus;
use hartex_bors_database::DatabaseClient;
use hartex_log::log;

const TRY_MERGE_BRANCH_NAME: &str = "automation/bors/try-merge";

pub async fn try_command<C: RepositoryClient>(
    repository: &mut GithubRepositoryState<C>,
    database: &mut dyn DatabaseClient,
    pr: u64,
    author: &str,
) -> hartex_eyre::Result<()> {
    if !check_try_permissions(repository, pr, author).await? {
        return Ok(());
    }

    let pr_model = database
        .get_or_create_pull_request(repository.client.repository_name(), pr)
        .await?;

    if let Some(ref build) = pr_model.try_build && build.status == BorsBuildStatus::Pending {
        repository
            .client
            .post_comment(pr, ":warning: A try build is currently in progress. You can cancel the build using `bors try-`")
            .await?;

        return Ok(());
    };

    Ok(())
}

async fn check_try_permissions<C: RepositoryClient>(
    repository: &mut GithubRepositoryState<C>,
    pr: u64,
    author: &str,
) -> hartex_eyre::Result<bool> {
    log::trace!("checking try permissions");

    let result = if !repository
        .permission_resolver
        .resolve_user(author, Permission::TryBuild)
        .await
    {
        log::warn!("user does not have try permisisons");

        repository
            .client
            .post_comment(
                pr,
                &format!(":lock: @{author}, you do not have the necessary privileges to run this command.")
            ).await?;

        false
    } else {
        true
    };

    Ok(result)
}
