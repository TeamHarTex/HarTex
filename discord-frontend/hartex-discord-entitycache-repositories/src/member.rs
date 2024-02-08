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

use hartex_database_queries::discord_frontend::queries::cached_member_select_by_user_id_and_guild_id::cached_member_select_by_user_id_and_guild_id;
use hartex_database_queries::discord_frontend::queries::cached_member_upsert::cached_member_upsert;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::member::MemberEntity;
use hartex_discord_utils::DATABASE_POOL;
use tokio_postgres::GenericClient;

pub struct CachedMemberRepository;

impl Repository<MemberEntity> for CachedMemberRepository {
    async fn get(
        &self,
        (guild_id, user_id): <MemberEntity as Entity>::Id,
    ) -> CacheResult<MemberEntity> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        let data = cached_member_select_by_user_id_and_guild_id()
            .bind(client, &user_id.to_string(), &guild_id.to_string())
            .one()
            .await?;

        Ok(MemberEntity {
            nick: data.nick,
            roles: data
                .roles
                .into_iter()
                .map(|role| Id::from_str(&role).unwrap())
                .collect(),
            guild_id: Id::from_str(&data.guild_id).unwrap(),
            user_id: Id::from_str(&data.user_id).unwrap(),
        })
    }

    async fn upsert(&self, entity: MemberEntity) -> CacheResult<()> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        cached_member_upsert()
            .bind(
                client,
                &entity.nick,
                &entity.user_id.to_string(),
                &entity.guild_id.to_string(),
                &entity
                    .roles
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )
            .await?;

        Ok(())
    }
}
