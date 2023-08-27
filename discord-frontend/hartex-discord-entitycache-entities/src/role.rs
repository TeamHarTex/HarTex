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

use hartex_discord_core::discord::model::guild::Role;
use hartex_discord_core::discord::model::guild::RoleFlags;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;
use hartex_discord_entitycache_core::Entity;

#[allow(clippy::module_name_repetitions)]
#[derive(Entity)]
pub struct RoleEntity {
    pub color: u32,
    pub flags: RoleFlags,
    #[entity(id)]
    pub guild_id: Id<GuildMarker>,
    pub hoist: bool,
    pub icon: Option<ImageHash>,
    #[entity(id)]
    pub id: Id<RoleMarker>,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub position: i64,
}

impl From<(Id<GuildMarker>, Role)> for RoleEntity {
    fn from((guild_id, role): (Id<GuildMarker>, Role)) -> Self {
        Self {
            color: role.color,
            flags: role.flags,
            guild_id,
            hoist: role.hoist,
            icon: role.icon,
            id: role.id,
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            position: role.position,
        }
    }
}
