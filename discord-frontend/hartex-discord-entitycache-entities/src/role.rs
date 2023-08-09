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
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_entitycache_core::Entity;

#[allow(clippy::module_name_repetitions)]
#[derive(Entity)]
pub struct RoleEntity {
    #[entity(id)]
    pub guild_id: Id<GuildMarker>,
    #[entity(id)]
    pub id: Id<RoleMarker>,
    pub name: String,
}

impl From<(Id<GuildMarker>, Role)> for RoleEntity {
    fn from((guild_id, role): (Id<GuildMarker>, Role)) -> Self {
        Self {
            guild_id,
            id: role.id,
            name: role.name,
        }
    }
}
