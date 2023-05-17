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

use std::time::Duration;
use std::time::SystemTime;

use crate::permissions::UserPermissions;

const CACHED_DURATION: Duration = Duration::from_secs(60);

pub(crate) struct CachedUserPermissions {
    created_at: SystemTime,
    permissions: UserPermissions,
}

impl CachedUserPermissions {
    pub(crate) fn new(permissions: UserPermissions) -> Self {
        Self {
            created_at: SystemTime::now(),
            permissions,
        }
    }
    
    pub(crate) fn is_invalidated(&self) -> bool {
        self.created_at
            .elapsed()
            .map(|duration| duration > CACHED_DURATION)
            .unwrap_or(true)
    }
}
