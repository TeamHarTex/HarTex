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

//! # The `role` Module
//!
//! This module implements the guild role entity.

use hartex_base::{
    discord::model::{
        guild::{
            Permissions,
            Role,
            RoleTags
        },
        id::{
            GuildId,
            RoleId
        }
    },
    stdext::prelude::*
};

use crate::entity::Entity;

/// # Struct `GuildRoleEntity`
///
/// A guild role entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct RoleEntity {
    color: u32,
    guild_id: GuildId,
    hoist: bool,
    icon: Option<String>,
    id: RoleId,
    managed: bool,
    mentionable: bool,
    name: String,
    permissions: Permissions,
    tags: Option<RoleTags>,
    unicode_emoji: Option<String>
}

impl RoleEntity {
    #[must_use]
    pub fn color(&self) -> u32 {
        self.color
    }

    #[must_use]
    /// # Instance Method `guild_id`
    ///
    /// Returns the ID of the guild this role is associated to.
    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn hoist(&self) -> bool {
        self.hoist
    }

    #[must_use]
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_refstr()
    }

    #[must_use]
    pub fn managed(&self) -> bool {
        self.managed
    }

    #[must_use]
    pub fn mentionable(&self) -> bool {
        self.mentionable
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn permissions(&self) -> Permissions {
        self.permissions
    }

    #[must_use]
    pub fn tags(&self) -> Option<&RoleTags> {
        self.tags.as_ref()
    }

    #[must_use]
    pub fn unicode_emoji(&self) -> Option<&str> {
        self.unicode_emoji.as_refstr()
    }
}

impl Entity for RoleEntity {
    type Id = RoleId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<(Role, GuildId)> for RoleEntity {
    fn from((role, guild_id): (Role, GuildId)) -> Self {
        Self {
            color: role.color,
            guild_id,
            hoist: role.hoist,
            icon: role.icon,
            id: role.id,
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: role.permissions,
            tags: role.tags,
            unicode_emoji: role.unicode_emoji
        }
    }
}
