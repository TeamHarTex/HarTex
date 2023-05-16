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

//! # Bors Models V1
//!
//! Models for the bors API specifcation V1 of the backend.

use serde::Deserialize;

/// A response to a repository permissions query.
#[derive(Clone, Deserialize)]
pub struct RepositoryPermissionsResponse {
    github_users: Vec<String>,
}

impl RepositoryPermissionsResponse {
    /// The Github users having this specific permission.
    pub fn github_users(&self) -> &[String] {
        self.github_users.as_ref()
    }
}
