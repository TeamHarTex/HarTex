/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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
use chrono::DateTime;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::member::MemberEntity;
use hartex_database_queries::queries::discord_frontend::cached_member_select_by_user_id_and_guild_id::CachedMemberSelectByUserIdAndGuildId;
use hartex_database_queries::queries::discord_frontend::cached_member_upsert::CachedMemberUpsert;

/// Repository for member entities.
pub struct CachedMemberRepository;

impl Repository<MemberEntity> for CachedMemberRepository {
    #[allow(clippy::cast_sign_loss)]
    async fn get(
        &self,
        (guild_id, user_id): <MemberEntity as Entity>::Id,
    ) -> CacheResult<MemberEntity> {
        let data =
            CachedMemberSelectByUserIdAndGuildId::bind(user_id.to_string(), guild_id.to_string())
                .executor()
                .await?
                .one()
                .await?;

        Ok(MemberEntity::from(data))
    }

    #[allow(clippy::cast_possible_wrap)]
    async fn upsert(&self, entity: MemberEntity) -> CacheResult<()> {
        CachedMemberUpsert::bind(
            entity.flags.bits() as i64,
            entity
                .joined_at
                .map(|timestamp| DateTime::from_timestamp(timestamp.as_secs(), 0).unwrap())
                .unwrap(),
            entity.nick.unwrap_or_default(),
            entity.user_id.to_string(),
            entity.guild_id.to_string(),
            entity
                .roles
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        )
        .executor()
        .await?
        .execute()
        .await?;

        Ok(())
    }
}
