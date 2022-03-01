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
use base::discord::model::user::{CurrentUser, PremiumType, UserFlags};
use base::discord::model::util::ImageHash;
use cache_base::Entity;

/// This is basically identical to a regular cached user.
///
/// This structure is here only for a separate cache for the current user (aka the bot itself).
pub struct CachedCurrentUser {
    pub(in crate) accent_colour: Option<u64>,
    pub(in crate) avatar: Option<ImageHash>,
    pub(in crate) banner: Option<ImageHash>,
    pub(in crate) bot: bool,
    pub(in crate) discriminator: String,
    pub(in crate) email: Option<String>,
    pub(in crate) flags: Option<UserFlags>,
    pub(in crate) id: Id<UserMarker>,
    pub(in crate) locale: Option<String>,
    pub(in crate) mfa_enabled: bool,
    pub(in crate) username: String,
    pub(in crate) premium_type: Option<PremiumType>,
    pub(in crate) public_flags: Option<UserFlags>,
    pub(in crate) system: Option<bool>,
    pub(in crate) verified: Option<bool>,
}

impl CachedCurrentUser {
    pub fn accent_colour(&self) -> Option<u64> {
        self.accent_colour
    }

    pub fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    pub fn banner(&self) -> Option<ImageHash> {
        self.banner
    }

    pub fn bot(&self) -> bool {
        self.bot
    }

    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn flags(&self) -> Option<UserFlags> {
        self.flags
    }

    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    pub fn mfa_enabled(&self) -> bool {
        self.mfa_enabled
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn premium_type(&self) -> Option<PremiumType> {
        self.premium_type
    }

    pub fn public_flags(&self) -> Option<UserFlags> {
        self.public_flags
    }

    pub fn system(&self) -> Option<bool> {
        self.system
    }

    pub fn verified(&self) -> Option<bool> {
        self.verified
    }
}

impl Entity for CachedCurrentUser {
    type Id = Id<UserMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<CurrentUser> for CachedCurrentUser {
    fn from(other: CurrentUser) -> Self {
        Self {
            accent_colour: other.accent_color,
            avatar: other.avatar,
            banner: other.banner,
            bot: other.bot,
            discriminator: other.discriminator.to_string(),
            email: other.email,
            flags: other.flags,
            id: other.id,
            locale: other.locale,
            mfa_enabled: other.mfa_enabled,
            username: other.name,
            premium_type: other.premium_type,
            public_flags: other.public_flags,
            system: Some(false),
            verified: other.verified,
        }
    }
}

#[cfg(postgres)]
include!("postgres_backend_include/users.rs");
