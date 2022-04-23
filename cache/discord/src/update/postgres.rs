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

use base::discord::model::gateway::payload::incoming::{GuildCreate, Ready};
use cache_base::Repository;

use crate::entities::guilds::CachedGuild;
use crate::entities::users::CachedCurrentUser;
use crate::postgres::PostgresBackend;
use crate::repositories::{CurrentUserRepository, GuildRepository};

impl CacheUpdatable<PostgresBackend> for GuildCreate {
    fn update(&self, _: &DiscordCache) -> UpdateCacheFuture<'_, PostgresBackend> {
        let guild = CachedGuild::from(self.0.clone());

        GuildRepository.upsert(guild)
    }
}

impl CacheUpdatable<PostgresBackend> for Ready {
    fn update(&self, _: &DiscordCache) -> UpdateCacheFuture<'_, crate::postgres::PostgresBackend> {
        let current_user = CachedCurrentUser::from(self.user.clone());

        CurrentUserRepository.upsert(current_user)
    }
}
