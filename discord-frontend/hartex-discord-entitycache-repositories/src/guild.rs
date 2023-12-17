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
use std::str::FromStr;

use hartex_database_queries::discord_frontend::queries::cached_guild_upsert::cached_guild_upsert;
use hartex_database_queries::discord_frontend::queries::cached_guild_select_by_id::cached_guild_select_by_id;
use hartex_discord_core::discord::model::guild::DefaultMessageNotificationLevel;
use hartex_discord_core::discord::model::guild::ExplicitContentFilter;
use hartex_discord_core::discord::model::guild::GuildFeature;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::guild::GuildEntity;
use tokio_postgres::NoTls;

/// Repository for guild entities.
pub struct CachedGuildRepository;

impl Repository<GuildEntity> for CachedGuildRepository {
    async fn get(&self, id: <GuildEntity as Entity>::Id) -> CacheResult<GuildEntity> {
        let (client, _) = tokio_postgres::connect(&env::var("HARTEX_NIGHTLY_PGSQL_URL")?, NoTls).await?;

        let data = cached_guild_select_by_id().bind(&client, &id.to_string()).one().await?;

        Ok(GuildEntity {
            default_message_notifications: DefaultMessageNotificationLevel::from(
                data.default_message_notifications as u8,
            ),
            explicit_content_filter: ExplicitContentFilter::from(data.explicit_content_filter as u8),
            features: data.features.iter().cloned().map(|feature| GuildFeature::from(feature)).collect(),
            icon: data.icon.map(|hash| ImageHash::parse(hash.as_bytes()).unwrap()),
            id,
            large: data.large,
            name: data.name,
            owner_id: Id::from_str(&data.owner_id).expect("id is zero (unexpected and unreachable)"),
        })
    }

    async fn upsert(&self, entity: GuildEntity) -> CacheResult<()> {
        let (client, _) = tokio_postgres::connect(&env::var("HARTEX_NIGHTLY_PGSQL_URL")?, NoTls).await?;

        cached_guild_upsert().bind(
            &client,
            &i32::from(<DefaultMessageNotificationLevel as Into<u8>>::into(entity.default_message_notifications)),
            &i32::from(<ExplicitContentFilter as Into<u8>>::into(entity.explicit_content_filter)),
            &entity.features.iter().map(|feature| feature.clone().into()).collect::<Vec<Cow<'static, str>>>(),
            &entity.icon.map(|hash| hash.to_string()),
            &entity.large,
            &entity.name,
            &entity.owner_id.to_string(),
            &entity.id.to_string()
        ).await?;

        Ok(())
    }
}
