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

//! Checking Bors Permissions

use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::models::Permission;
use hartex_bors_core::RepositoryClient;
use hartex_log::log;

pub async fn check_permissions<C: RepositoryClient>(
    repository: &GithubRepositoryState<C>,
    pr: u64,
    author: &str,
    permission: Permission,
) -> miette::Result<bool> {
    log::trace!("checking {permission} permissions");

    let result = if !repository
        .permission_resolver
        .resolve_user(author, permission)
        .await
    {
        log::warn!("user does not have {permission} permisisons");

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
