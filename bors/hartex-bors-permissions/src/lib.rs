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

//! # Bors Permission Resolver

#![feature(async_fn_in_trait)]

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use hartex_bors_github::models::GithubRepositoryName;
use hartex_log::log;
use tokio::sync::Mutex;

use crate::cached::CachedUserPermissions;

pub mod cached;
pub mod permissions;

/// The type of permission.
#[non_exhaustive]
pub enum Permission {
    /// Permission to try builds.
    ///
    /// @bors try
    TryBuild,
}

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::TryBuild => write!(f, "trybuild"),
            _ => write!(f, "unknown"),
        }
    }
}

pub struct BackendApiPermissionResolver {
    repository: GithubRepositoryName,
    permissions: Mutex<CachedUserPermissions>,
}

impl BackendApiPermissionResolver {
    pub async fn load(repository: GithubRepositoryName) -> hartex_eyre::Result<Self> {
        let permissions = crate::permissions::load(&repository).await?;

        Ok(Self {
            repository,
            permissions: Mutex::new(CachedUserPermissions::new(permissions))
        })
    }

    async fn reload(&self) {
        let result = crate::permissions::load(&self.repository).await;
        match result {
            Ok(perms) => *self.permissions.lock().await = CachedUserPermissions::new(perms),
            Err(error) => {
                log::error!("Cannot reload permissions for {}: {error:?}", self.repository);
            }
        }
    }
}

/// A base permission resolver.
pub trait PermissionResolver {
    /// Resolves permissions for a user and returns whether that user has the specified permission.
    async fn resolve_user(&self, username: &str, permission: Permission) -> bool;
}
