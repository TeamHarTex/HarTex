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

use hartex_database_queries::discord_frontend::queries::cached_emoji_select_by_id::cached_emoji_select_by_id;
use hartex_database_queries::discord_frontend::queries::cached_emoji_upsert::cached_emoji_upsert;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::emoji::EmojiEntity;
use hartex_discord_utils::DATABASE_POOL;
use tokio_postgres::GenericClient;

/// Repository for emoji entities.
pub struct CachedEmojiRepository;

impl Repository<EmojiEntity> for CachedEmojiRepository {
    async fn get(&self, id: <EmojiEntity as Entity>::Id) -> CacheResult<EmojiEntity> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        let data = cached_emoji_select_by_id()
            .bind(client, &id.to_string())
            .one()
            .await?;

        Ok(EmojiEntity::from(data))
    }

    async fn upsert(&self, entity: EmojiEntity) -> CacheResult<()> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        cached_emoji_upsert()
            .bind(
                client,
                &entity.animated,
                &entity.id.to_string(),
                &entity.guild_id.to_string(),
                &entity.name,
                &entity.managed,
            )
            .await?;

        Ok(())
    }
}
