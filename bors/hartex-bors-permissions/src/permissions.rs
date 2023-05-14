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

use std::collections::HashSet;

use hartex_bors_github::models::GithubRepositoryName;

use crate::Permission;

/// User permissions data structure.
pub struct UserPermissions {
    try_build_users: HashSet<String>,
}

impl UserPermissions {
    /// Checks whether a user has a certain permission.
    pub fn user_has_permission(&self, username: &str, permission: Permission) -> bool {
        match permission {
            Permission::TryBuild => self.try_build_users.contains(username),
            _ => false,
        }
    }
}

async fn load(repository: &GithubRepositoryName) -> hartex_eyre::Result<UserPermissions> {
    let try_build_users = load_permissions_from_api(repository.repository(), Permission::TryBuild).await?;

    todo!()
}

async fn load_permissions_from_api(repository_name: &str, permission: Permission) -> hartex_eyre::Result<HashSet<String>> {
    todo!()
}
