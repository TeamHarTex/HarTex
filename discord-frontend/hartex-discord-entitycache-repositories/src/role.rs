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
use std::str::FromStr;

use hartex_database_queries::discord_frontend::queries::cached_role_select_by_guild_id::cached_role_select_by_guild_id;
use hartex_database_queries::discord_frontend::queries::cached_role_select_by_id_and_guild_id::cached_role_select_by_id_and_guild_id;
use hartex_database_queries::discord_frontend::queries::cached_role_upsert::cached_role_upsert;
use hartex_discord_core::discord::model::guild::RoleFlags;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::role::RoleEntity;
use hartex_discord_utils::DATABASE_POOL;
use tokio_postgres::GenericClient;

pub struct CachedRoleRepository;

impl CachedRoleRepository {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub async fn role_ids_in_guild(&self, guild_id: Id<GuildMarker>) -> CacheResult<Vec<Id<RoleMarker>>> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        let roles = cached_role_select_by_guild_id()
            .bind(client, &guild_id.to_string())
            .all()
            .await?;

        Ok(roles.into_iter().map(|role| Id::<RoleMarker>::from_str(&role.id).unwrap()).collect())
    }
}

impl Repository<RoleEntity> for CachedRoleRepository {
    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    async fn get(&self, (guild_id, id): <RoleEntity as Entity>::Id) -> CacheResult<RoleEntity> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        let data = cached_role_select_by_id_and_guild_id()
            .bind(client, &id.to_string(), &guild_id.to_string())
            .one()
            .await?;

        Ok(RoleEntity {
            color: data.color as u32,
            flags: RoleFlags::from_bits_truncate(data.flags as u64),
            guild_id,
            hoist: data.hoist,
            icon: data
                .icon
                .map(|hash| ImageHash::parse(hash.as_bytes()).unwrap()),
            id,
            managed: data.managed,
            mentionable: data.mentionable,
            position: data.position as i64,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    async fn upsert(&self, entity: RoleEntity) -> CacheResult<()> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        cached_role_upsert()
            .bind(
                client,
                &(entity.color as i64),
                &entity.icon.map(|hash| hash.to_string()),
                &entity.id.to_string(),
                &entity.guild_id.to_string(),
                &(entity.flags.bits() as i32),
                &entity.hoist,
                &entity.managed,
                &entity.mentionable,
                &(entity.position as i32),
            )
            .await?;

        Ok(())
    }
}
