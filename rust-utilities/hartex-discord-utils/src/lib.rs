/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # Discord Utilities
//!
//! Various useful Discord utilities.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(const_async_blocks)]
#![feature(type_alias_impl_trait)]

use std::env;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Duration;

use async_lazy::Lazy;
use hartex_discord_core::discord::http::Client;
use log::LevelFilter;
use sqlx::ConnectOptions;
use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;

pub mod commands;
pub mod hyper;
pub mod interaction;
pub mod localizable;
pub mod markdown;
pub mod postgres;

/// A proxied Discord HTTP cliemt.
pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .token(TOKEN.deref().to_owned())
        .proxy(String::from("localhost:3000"), true)
        .ratelimiter(None)
        .build()
});

/// An asynchronously lazyily initialized database pool.
pub static DATABASE_POOL: Lazy<PgPool> = Lazy::new(|| Box::pin(async {
    let hartex_pgsql_url = env::var("DISCORD_FRONTEND_PGSQL_URL").unwrap();
    let options = PgConnectOptions::from_str(&hartex_pgsql_url)
        .unwrap()
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Debug, Duration::from_secs(1));
    PgPoolOptions::new().connect_with(options).await.unwrap()
}));

/// The bot token used for logging in to the Discord gateway and sending HTTP requests.
pub static TOKEN: LazyLock<String> = LazyLock::new(|| env::var("BOT_TOKEN").unwrap());
