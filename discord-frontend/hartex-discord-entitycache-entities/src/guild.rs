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

//! # Guild Entities

use hartex_discord_core::discord::model::guild::Guild;
use hartex_discord_core::discord::model::guild::GuildFeature;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::UserMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;
use hartex_discord_entitycache_core::Entity;

#[allow(clippy::module_name_repetitions)]
#[derive(Entity)]
pub struct GuildEntity {
    pub features: Vec<GuildFeature>,
    pub icon: Option<ImageHash>,
    #[entity(id)]
    pub id: Id<GuildMarker>,
    pub large: bool,
    pub name: String,
    pub owner_id: Id<UserMarker>,
}

impl From<Guild> for GuildEntity {
    fn from(guild: Guild) -> Self {
        Self {
            features: guild.features,
            icon: guild.icon,
            id: guild.id,
            large: guild.large,
            name: guild.name,
            owner_id: guild.owner_id,
        }
    }
}
