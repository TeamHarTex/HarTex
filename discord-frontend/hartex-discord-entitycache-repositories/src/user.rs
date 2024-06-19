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

use std::pin::Pin;

use hartex_database_queries::discord_frontend::queries::cached_user_select_by_id::cached_user_select_by_id;
use hartex_database_queries::discord_frontend::queries::cached_user_upsert::cached_user_upsert;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::user::UserEntity;
use hartex_discord_utils::DATABASE_POOL;
use tokio_postgres::GenericClient;

/// Repository for user entities.
pub struct CachedUserRepository;

impl Repository<UserEntity> for CachedUserRepository {
    async fn get(&self, id: <UserEntity as Entity>::Id) -> CacheResult<UserEntity> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        let data = cached_user_select_by_id()
            .bind(client, &id.to_string())
            .one()
            .await?;

        Ok(UserEntity::from(data))
    }

    async fn upsert(&self, entity: UserEntity) -> CacheResult<()> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        cached_user_upsert()
            .bind(
                client,
                &entity.avatar.map(|hash| hash.to_string()),
                &entity.id.to_string(),
                &entity.bot,
                &entity.name,
                &entity.discriminator.to_string(),
                &entity.global_name,
            )
            .await?;

        Ok(())
    }
}
