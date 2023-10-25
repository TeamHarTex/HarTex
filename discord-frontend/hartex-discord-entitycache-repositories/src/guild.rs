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

//! # Guild Repository

use std::borrow::Cow;
use std::env;

use hartex_discord_core::discord::model::guild::DefaultMessageNotificationLevel;
use hartex_discord_core::discord::model::guild::ExplicitContentFilter;
use hartex_discord_core::discord::model::guild::GuildFeature;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::guild::GuildEntity;
use redis::AsyncCommands;
use redis::Client;

/// Repository for guild entities.
pub struct CachedGuildRepository;

impl Repository<GuildEntity> for CachedGuildRepository {
    async fn get(&self, id: <GuildEntity as Entity>::Id) -> CacheResult<GuildEntity> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        let default_message_notifications = connection
            .get::<String, u8>(format!("guild:{id}:default_message_notifications"))
            .await?;
        let explicit_content_filter = connection
            .get::<String, u8>(format!("guild:{id}:explicit_content_filter"))
            .await?;
        let features = connection
            .get::<String, String>(format!("guild:{id}:features"))
            .await?
            .split(',')
            .map(|str| GuildFeature::from(str.to_string()))
            .collect::<Vec<_>>();
        let icon = connection
            .get::<String, Option<String>>(format!("guild:{id}:icon"))
            .await?;
        let large = connection
            .get::<String, bool>(format!("guild:{id}:large"))
            .await?;
        let name = connection
            .get::<String, String>(format!("guild:{id}:name"))
            .await?;
        let owner_id = connection
            .get::<String, u64>(format!("guild:{id}:owner_id"))
            .await?;

        Ok(GuildEntity {
            default_message_notifications: DefaultMessageNotificationLevel::from(
                default_message_notifications,
            ),
            explicit_content_filter: ExplicitContentFilter::from(explicit_content_filter),
            features,
            icon: icon.map(|hash| ImageHash::parse(hash.as_bytes()).unwrap()),
            id,
            large,
            name,
            owner_id: Id::new_checked(owner_id).expect("id is zero (unexpected and unreachable)"),
        })
    }

    async fn upsert(&self, entity: GuildEntity) -> CacheResult<()> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        if let Some(icon) = entity.icon {
            connection
                .set(format!("guild:{}:icon", entity.id), icon.to_string())
                .await?;
        }

        connection
            .set(
                format!("guild:{}:default_message_notifications", entity.id),
                u8::from(entity.default_message_notifications),
            )
            .await?;
        connection
            .set(
                format!("guild:{}:explicit_content_filter", entity.id),
                u8::from(entity.explicit_content_filter),
            )
            .await?;
        connection
            .set(
                format!("guild:{}:features", entity.id),
                entity
                    .features
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<Cow<'static, str>>>()
                    .join(","),
            )
            .await?;
        connection
            .set(format!("guild:{}:large", entity.id), entity.large)
            .await?;
        connection
            .set(format!("guild:{}:name", entity.id), entity.name)
            .await?;
        connection
            .set(
                format!("guild:{}:owner_id", entity.id),
                entity.owner_id.get(),
            )
            .await?;

        Ok(())
    }
}
