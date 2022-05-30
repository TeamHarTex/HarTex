/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! Users in the Discord entity cache.

use base::discord::model::id::{marker::UserMarker, Id};
use base::discord::model::user::{CurrentUser, UserFlags};
use base::discord::model::util::ImageHash;
use cache_base::Entity;

/// This is basically identical to a regular cached user.
///
/// This structure is here only for a separate cache for the current user (aka the bot itself).
#[derive(Clone)]
pub struct CachedCurrentUser {
    pub(crate) avatar: Option<ImageHash>,
    pub(crate) discriminator: String,
    pub(crate) flags: Option<UserFlags>,
    pub(crate) id: Id<UserMarker>,
    pub(crate) username: String,
    pub(crate) public_flags: Option<UserFlags>,
}

impl CachedCurrentUser {
    #[must_use]
    pub fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    #[must_use]
    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }

    #[must_use]
    pub fn flags(&self) -> Option<UserFlags> {
        self.flags
    }

    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    #[must_use]
    pub fn public_flags(&self) -> Option<UserFlags> {
        self.public_flags
    }
}

impl Entity for CachedCurrentUser {
    type Id = Id<UserMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<CurrentUser> for CachedCurrentUser {
    fn from(from: CurrentUser) -> Self {
        Self {
            avatar: from.avatar,
            discriminator: from.discriminator.to_string(),
            flags: from.flags,
            id: from.id,
            username: from.name,
            public_flags: from.public_flags,
        }
    }
}

#[cfg(postgres)]
include!("postgres_backend_include/users.rs");
