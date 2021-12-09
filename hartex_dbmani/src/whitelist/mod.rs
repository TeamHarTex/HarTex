/* SPDX-License-Identifier: GPL-2.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # The `whitelist` Module
//!
//! This module defines a database manipulation procedure for obtaining the whitelisted guilds of
//! the bot for checking whitelists.

use std::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use hartex_core::{
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing
};
use sqlx::{
    postgres::PgPool,
    Postgres
};

use crate::{
    whitelist::model::WhitelistedGuild,
    PendingFuture,
    DATABASE_ENV
};

mod model;

/// # Struct `GetWhitelistedGuilds`
///
/// Gets the whitelisted guilds of the bot.
#[derive(Default)]
pub struct GetWhitelistedGuilds {
    pending: Option<PendingFuture<Vec<WhitelistedGuild>>>
}

impl GetWhitelistedGuilds {
    /// # Private Function `GetWhitelistedGuilds::start`
    ///
    /// Starts the future.
    #[allow(clippy::unnecessary_wraps)]
    fn start(&mut self) -> HarTexResult<()> {
        let span = tracing::trace_span!(
            parent: None,
            "database manipulation: get whitelisted guilds"
        );
        span.in_scope(|| tracing::trace!("executing future `GetWhitelistedGuilds`"));

        self.pending.replace(Box::pin(exec_future()));

        Ok(())
    }
}

impl Future for GetWhitelistedGuilds {
    type Output = HarTexResult<Vec<WhitelistedGuild>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(pending) = self.pending.as_mut() {
                return pending.as_mut().poll(cx);
            }

            if let Err(error) = self.start() {
                return Poll::Ready(Err(error));
            }
        }
    }
}

/// # Asynchronous Function `exec_future`
///
/// Executes the future.
async fn exec_future() -> HarTexResult<Vec<WhitelistedGuild>> {
    let span = tracing::trace_span!(
        parent: None,
        "database manipulation: get whitelisted guilds"
    );

    let db_credentials = match &DATABASE_ENV.pgsql_credentials_guilds {
        Some(credentials) => credentials,
        None => {
            span.in_scope(|| {
                tracing::error!("the `PGSQL_CREDENTIALS_GUILDS` environment variable is not set");
            });

            return Err(HarTexError::Custom {
                message: String::from(
                    "the `PGSQL_CREDENTIALS_GUILDS` environment variable is not set"
                )
            });
        }
    };

    span.in_scope(|| tracing::trace!("connecting to database..."));

    let connection = match PgPool::connect(db_credentials).await {
        Ok(pool) => pool,
        Err(error) => {
            let message =
                format!("failed to connect to postgres database pool; error: `{error:?}`");

            span.in_scope(|| tracing::error!("{message}", message = &message));

            return Err(HarTexError::Custom {
                message
            });
        }
    };

    span.in_scope(|| tracing::trace!("executing sql query..."));

    match sqlx::query_as::<Postgres, WhitelistedGuild>(r#"SELECT * FROM public."Whitelist"; --"#)
        .fetch_all(&connection)
        .await
    {
        Ok(guilds) => Ok(guilds),
        Err(error) => {
            let message = format!("failed to execute sql query; error: `{error:?}`");

            span.in_scope(|| tracing::error!("{message}", message = &message));

            Err(HarTexError::Custom {
                message
            })
        }
    }
}
