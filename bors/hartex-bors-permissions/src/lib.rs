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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(async_fn_in_trait)]

use std::future::Future;
use std::pin::Pin;

use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::Permission;
use hartex_bors_core::PermissionResolver;
use hartex_log::log;
use tokio::sync::Mutex;

use crate::cached::CachedUserPermissions;

pub mod cached;
pub mod permissions;

/// Backend API permission resolver
pub struct BackendApiPermissionResolver {
    repository: GithubRepositoryName,
    permissions: Mutex<CachedUserPermissions>,
}

impl BackendApiPermissionResolver {
    /// Load the permission resolver
    pub async fn load(repository: GithubRepositoryName) -> hartex_eyre::Result<Self> {
        let permissions = permissions::load(&repository).await?;

        Ok(Self {
            repository,
            permissions: Mutex::new(CachedUserPermissions::new(permissions)),
        })
    }

    async fn reload(&self) {
        let result = permissions::load(&self.repository).await;
        match result {
            Ok(perms) => *self.permissions.lock().await = CachedUserPermissions::new(perms),
            Err(error) => {
                log::error!(
                    "Cannot reload permissions for {}: {error:?}",
                    self.repository
                );
            }
        }
    }
}

impl PermissionResolver for BackendApiPermissionResolver {
    fn resolve_user<'a>(
        &'a self,
        username: &'a str,
        permission: Permission,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move {
            if self.permissions.lock().await.is_invalidated() {
                self.reload().await;
            }

            self.permissions
                .lock()
                .await
                .permissions
                .user_has_permission(username, permission)
        })
    }
}
