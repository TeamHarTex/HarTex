/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

use hartex_discord_core::discord::model::gateway::payload::incoming::MemberChunk;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::member::MemberEntity;
use hartex_discord_entitycache_entities::user::UserEntity;
use hartex_discord_entitycache_repositories::member::CachedMemberRepository;
use hartex_discord_entitycache_repositories::user::CachedUserRepository;

use crate::CacheUpdater;

impl CacheUpdater for MemberChunk {
    async fn update(&self) -> CacheResult<()> {
        for member in &self.members {
            let member_entity = MemberEntity::from((self.guild_id, member.user.id, member.clone()));
            let user_entity = UserEntity::from(member.user.clone());

            CachedMemberRepository.upsert(member_entity).await?;
            CachedUserRepository.upsert(user_entity).await?;
        }

        Ok(())
    }
}
