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

//! Guild whitelist manipulation procedures.

use std::env;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use base::discord::model::id::{marker::GuildMarker, Id};
use base::error::{Error, Result};
use model::db::whitelist::WhitelistedGuild;
use sqlx::{Error as SqlxError, PgPool, Postgres};

use crate::Pending;

pub struct GetGuildWhitelistStatus {
    future: Option<Pending<Option<WhitelistedGuild>>>,
    guild_id: Id<GuildMarker>,
}

impl GetGuildWhitelistStatus {
    #[must_use]
    pub fn new(guild_id: Id<GuildMarker>) -> Self {
        Self {
            future: None,
            guild_id,
        }
    }

    fn launch(&mut self) {
        log::trace!("launching future `GetGuildWhitelistStatus`");

        self.future.replace(Box::pin(exec(self.guild_id)));
    }
}

impl Future for GetGuildWhitelistStatus {
    type Output = Result<Option<WhitelistedGuild>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(future) = self.future.as_mut() {
                return future.as_mut().poll(cx);
            }

            self.launch();
        }
    }
}

async fn exec(guild_id: Id<GuildMarker>) -> Result<Option<WhitelistedGuild>> {
    #[cfg(stable)]
    let pgsql_creds = env::var("PGSQL_WHITELIST_DB_CREDENTIALS").map_err(|src| {
        log::error!(
            "could not retrieve `PGSQL_WHITELIST_DB_CREDENTIALS` environment variable: {src}"
        );
        Error::from(src)
    })?;
    #[cfg(not(stable))]
        let pgsql_creds = env::var("PGSQL_WHITELIST_DB_NIGHTLY_CREDENTIALS").map_err(|src| {
        log::error!(
            "could not retrieve `PGSQL_WHITELIST_DB_NIGHTLY_CREDENTIALS` environment variable: {src}"
        );
        Error::from(src)
    })?;

    let pool = PgPool::connect(&pgsql_creds).await.map_err(|src| {
        log::error!("could not connect to whitelist database: {src}");
        Error::from(src)
    })?;
    let query = include_str!("../include/get_guild_whitelist_status.sql");

    let result = sqlx::query_as::<Postgres, WhitelistedGuild>(query)
        .bind(guild_id.to_string())
        .fetch_one(&pool)
        .await;

    if let Err(error) = &result && matches!(error, SqlxError::RowNotFound) {
        return Ok(None);
    }

    Ok(Some(result.map_err(|src| {
        log::error!("could not execute query: {src}");
        Error::from(src)
    })?))
}
