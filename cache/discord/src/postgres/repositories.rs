/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! Repositories in the PostgreSQL backend.

use std::env;

use base::discord::model::id::{marker::UserMarker, Id};
use cache_base::future::{GetEntityFuture, UpsertEntityFuture};
use cache_base::Repository;
use sqlx::postgres::PgPool;
use sqlx::Postgres;

use crate::entities::users::CachedCurrentUser;
use crate::postgres::error::PostgresBackendError;
use crate::postgres::PostgresBackend;

pub struct CurrentUserRepository;

impl Repository<PostgresBackend, CachedCurrentUser> for CurrentUserRepository {
    fn get(
        &self,
        entity_id: Id<UserMarker>,
    ) -> GetEntityFuture<'_, CachedCurrentUser, PostgresBackendError> {
        Box::pin(async move {
            let pgsql_creds = env::var("PGSQL_CACHE_DB_CREDENTIALS")?;
            let pool = PgPool::connect(&pgsql_creds).await?;
            let query = include_str!("../../include/postgres/repositories/current_user/get.sql");

            sqlx::query_as::<Postgres, CachedCurrentUser>(query)
                .bind(entity_id.to_string())
                .fetch_one(&pool)
                .await
                .map_err(PostgresBackendError::from)
        })
    }

    fn upsert(
        &self,
        current_user: CachedCurrentUser,
    ) -> UpsertEntityFuture<'_, PostgresBackendError> {
        Box::pin(async move {
            let pgsql_creds = env::var("PGSQL_CACHE_DB_CREDENTIALS")?;
            let pool = PgPool::connect(&pgsql_creds).await?;
            let query = include_str!("../../include/postgres/repositories/current_user/upsert.sql");

            sqlx::query(query)
                .bind(current_user.id.to_string())
                .bind(current_user.username)
                .bind(current_user.discriminator)
                .bind(current_user.avatar.map(|hash| hash.to_string()))
                .bind(current_user.flags.map(|flags| flags.bits().to_string()))
                .bind(
                    current_user
                        .public_flags
                        .map(|flags| flags.bits().to_string()),
                )
                .execute(&pool)
                .await?;

            Ok(())
        })
    }
}
