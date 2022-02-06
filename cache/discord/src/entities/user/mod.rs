/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
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

//! Entities related to Discord users.

use hartex_base::{
    discord::model::{
        id::{marker::UserMarker, Id},
        user::{PremiumType, User, UserFlags},
        util::ImageHash,
    },
    stdext::prelude::*,
};
use hartex_cache_base::entity::Entity;

#[allow(clippy::module_name_repetitions)]
pub mod current_user;

/// A user entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct UserEntity {
    accent_color: Option<u64>,
    avatar: Option<ImageHash>,
    banner: Option<ImageHash>,
    bot: bool,
    discriminator: u16,
    email: Option<String>,
    flags: Option<UserFlags>,
    id: Id<UserMarker>,
    locale: Option<String>,
    mfa_enabled: Option<bool>,
    name: String,
    premium_type: Option<PremiumType>,
    public_flags: Option<UserFlags>,
    system: Option<bool>,
    verified: Option<bool>,
}

impl UserEntity {
    #[must_use]
    pub fn accent_color(&self) -> Option<u64> {
        self.accent_color
    }

    #[must_use]
    pub fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    #[must_use]
    pub fn banner(&self) -> Option<ImageHash> {
        self.banner
    }

    #[must_use]
    pub fn bot(&self) -> bool {
        self.bot
    }

    #[must_use]
    pub fn discriminator(&self) -> u16 {
        self.discriminator
    }

    #[must_use]
    pub fn email(&self) -> Option<&str> {
        self.email.as_refstr()
    }

    #[must_use]
    pub fn flags(&self) -> Option<UserFlags> {
        self.flags
    }

    #[must_use]
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_refstr()
    }

    #[must_use]
    pub fn mfa_enabled(&self) -> Option<bool> {
        self.mfa_enabled
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn premium_type(&self) -> Option<PremiumType> {
        self.premium_type
    }

    #[must_use]
    pub fn public_flags(&self) -> Option<UserFlags> {
        self.public_flags
    }

    #[must_use]
    pub fn system(&self) -> Option<bool> {
        self.system
    }

    #[must_use]
    pub fn verified(&self) -> Option<bool> {
        self.verified
    }
}

impl Entity for UserEntity {
    type Id = Id<UserMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<User> for UserEntity {
    fn from(user: User) -> Self {
        Self {
            accent_color: user.accent_color,
            avatar: user.avatar,
            banner: user.banner,
            bot: user.bot,
            discriminator: user.discriminator,
            email: user.email,
            flags: user.flags,
            id: user.id,
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type,
            public_flags: user.public_flags,
            system: user.system,
            verified: user.verified,
        }
    }
}
