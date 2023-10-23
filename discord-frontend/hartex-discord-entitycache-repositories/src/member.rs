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
use std::str::FromStr;

use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::marker::UserMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::member::MemberEntity;
use redis::AsyncCommands;
use redis::Client;

pub struct CachedMemberRepository;

impl CachedMemberRepository {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub async fn member_ids_in_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<Vec<Id<UserMarker>>> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut sync_connection = client.get_connection()?;
        let keys = redis::cmd("SCAN")
            .cursor_arg(0)
            .arg("MATCH")
            .arg(format!("guild:{guild_id}:member:*:user_id"))
            .arg("COUNT")
            .arg("1000")
            .clone()
            .iter::<String>(&mut sync_connection)?
            .collect::<Vec<_>>();

        let mut members = Vec::new();

        let mut connection = client.get_tokio_connection().await?;
        for key in keys {
            let id = connection.get::<String, u64>(key).await?;

            members.push(Id::new_checked(id).expect("id is zero (unexpected and unreachable)"));
        }

        Ok(members)
    }
}

impl Repository<MemberEntity> for CachedMemberRepository {
    async fn get(
        &self,
        (guild_id, user_id): <MemberEntity as Entity>::Id,
    ) -> CacheResult<MemberEntity> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        let roles = connection
            .get::<String, String>(format!("guild:{guild_id}:member:{user_id}:roles"))
            .await?
            .split(',')
            .map(|str| Id::<RoleMarker>::from_str(str).unwrap())
            .collect::<Vec<_>>();

        Ok(MemberEntity {
            roles,
            guild_id,
            user_id,
        })
    }

    async fn upsert(&self, entity: MemberEntity) -> CacheResult<()> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;
        connection
            .set(
                format!(
                    "guild:{}:member:{}:user_id",
                    entity.guild_id, entity.user_id
                ),
                entity.user_id.get(),
            )
            .await?;
        connection
            .set(
                format!("guild:{}:member:{}:roles", entity.guild_id, entity.user_id),
                entity
                    .roles
                    .into_iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .await?;

        Ok(())
    }
}
