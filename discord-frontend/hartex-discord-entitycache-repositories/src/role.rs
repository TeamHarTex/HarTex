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

use std::env;

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
use redis::Client;
use serde_scan::scan;
use tokio_postgres::NoTls;

pub struct CachedRoleRepository;

impl CachedRoleRepository {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn role_ids_in_guild(&self, guild_id: Id<GuildMarker>) -> CacheResult<Vec<Id<RoleMarker>>> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut sync_connection = client.get_connection()?;
        let keys = redis::cmd("SCAN")
            .cursor_arg(0)
            .arg("MATCH")
            .arg(format!("guild:{guild_id}:role:*:name"))
            .arg("COUNT")
            .arg("1000")
            .clone()
            .iter::<String>(&mut sync_connection)?
            .collect::<Vec<_>>();

        Ok(keys
            .iter()
            .map(|key| {
                let (_, role_id): (u64, u64) = scan!("guild:{}:role:{}:name" <- key).unwrap();

                Id::new_checked(role_id).expect("unreachable")
            })
            .collect())
    }
}

impl Repository<RoleEntity> for CachedRoleRepository {
    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    async fn get(&self, (guild_id, id): <RoleEntity as Entity>::Id) -> CacheResult<RoleEntity> {
        let (client, _) = tokio_postgres::connect(&env::var("HARTEX_NIGHTLY_PGSQL_URL")?, NoTls).await?;

        let data = cached_role_select_by_id_and_guild_id()
            .bind(&client, &id.to_string(), &guild_id.to_string())
            .one()
            .await?;

        Ok(RoleEntity {
            color: data.color as u32,
            flags: RoleFlags::from_bits_truncate(data.flags as u64),
            guild_id,
            hoist: data.hoist,
            icon: data.icon.map(|hash| ImageHash::parse(hash.as_bytes()).unwrap()),
            id,
            managed: data.managed,
            mentionable: data.mentionable,
            position: data.position as i64,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    async fn upsert(&self, entity: RoleEntity) -> CacheResult<()> {
        let (client, _) = tokio_postgres::connect(&env::var("HARTEX_NIGHTLY_PGSQL_URL")?, NoTls).await?;

        cached_role_upsert()
            .bind(
                &client,
                &(entity.color as i64),
                &entity.icon.map(|hash| hash.to_string()), 
                &entity.id.to_string(), 
                &entity.guild_id.to_string(), 
                &(entity.flags.bits() as i32), 
                &entity.hoist, 
                &entity.managed, 
                &entity.mentionable, 
                &(entity.position as i32)
            ).await?;

        Ok(())
    }
}
