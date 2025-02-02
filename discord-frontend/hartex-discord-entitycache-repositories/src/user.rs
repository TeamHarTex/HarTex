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

use hartex_database_queries::queries::discord_frontend::cached_user_select_by_id::CachedUserSelectById;
use hartex_database_queries::queries::discord_frontend::cached_user_upsert::CachedUserUpsert;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::user::UserEntity;

/// Repository for user entities.
pub struct CachedUserRepository;

impl Repository<UserEntity> for CachedUserRepository {
    async fn get(&self, id: <UserEntity as Entity>::Id) -> CacheResult<UserEntity> {
        let data = CachedUserSelectById::bind(id.to_string())
            .executor()
            .await?
            .one()
            .await?;

        Ok(UserEntity::from(data))
    }

    async fn upsert(&self, entity: UserEntity) -> CacheResult<()> {
        CachedUserUpsert::bind(
            entity
                .avatar
                .map(|hash| hash.to_string())
                .unwrap_or_default(),
            entity.id.to_string(),
            entity.bot,
            entity.name,
            entity.discriminator.to_string(),
            entity.global_name.unwrap_or_default(),
        )
        .executor()
        .await?
        .execute()
        .await?;

        Ok(())
    }
}
