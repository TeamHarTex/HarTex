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

use hartex_discord_core::discord::model::guild::RoleFlags;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::role::RoleEntity;
use redis::AsyncCommands;
use redis::Client;
use serde_scan::scan;

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
    async fn get(&self, (guild_id, id): <RoleEntity as Entity>::Id) -> CacheResult<RoleEntity> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        let color = connection
            .get::<String, u32>(format!("guild:{guild_id}:role:{id}:color"))
            .await?;
        let flags = connection
            .get::<String, u64>(format!("guild:{guild_id}:role:{id}:flags"))
            .await?;
        let hoist = connection
            .get::<String, bool>(format!("guild:{guild_id}:role:{id}:hoist"))
            .await?;
        let icon = connection
            .get::<String, Option<String>>(format!("guild:{guild_id}:role:{id}:icon"))
            .await?;
        let managed = connection
            .get::<String, bool>(format!("guild:{guild_id}:role:{id}:managed"))
            .await?;
        let mentionable = connection
            .get::<String, bool>(format!("guild:{guild_id}:role:{id}:mentionable"))
            .await?;
        let position = connection
            .get::<String, i64>(format!("guild:{guild_id}:role:{id}:position"))
            .await?;

        Ok(RoleEntity {
            color,
            flags: RoleFlags::from_bits_truncate(flags),
            guild_id,
            hoist,
            icon: icon.map(|hash| ImageHash::parse(hash.as_bytes()).unwrap()),
            id,
            managed,
            mentionable,
            position,
        })
    }

    async fn upsert(&self, entity: RoleEntity) -> CacheResult<()> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        if let Some(icon) = entity.icon {
            connection
                .set(
                    format!("guild:{}:role:{}:icon", entity.guild_id, entity.id),
                    icon.to_string(),
                )
                .await?;
        }

        connection
            .set(
                format!("guild:{}:role:{}:color", entity.guild_id, entity.id),
                entity.color,
            )
            .await?;
        connection
            .set(
                format!("guild:{}:role:{}:flags", entity.guild_id, entity.id),
                entity.flags.bits(),
            )
            .await?;
        connection
            .set(
                format!("guild:{}:role:{}:hoist", entity.guild_id, entity.id),
                entity.hoist,
            )
            .await?;
        connection
            .set(
                format!("guild:{}:role:{}:managed", entity.guild_id, entity.id),
                entity.managed,
            )
            .await?;
        connection
            .set(
                format!("guild:{}:role:{}:mentionable", entity.guild_id, entity.id),
                entity.mentionable,
            )
            .await?;
        connection
            .set(
                format!("guild:{}:role:{}:position", entity.guild_id, entity.id),
                entity.position,
            )
            .await?;

        Ok(())
    }
}
