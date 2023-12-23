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

//! # Discord Utilities
//!
//! Various useful Discord utilities.

#![feature(const_async_blocks)]
#![feature(type_alias_impl_trait)]

use std::env;
use std::future::Future;
use std::ops::Deref;

use async_once_cell::Lazy as AsyncLazy;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use hartex_discord_core::discord::http::Client;
use once_cell::sync::Lazy;
use tokio_postgres::NoTls;

pub mod localizable;
pub mod markdown;

/// A proxied Discord HTTP cliemt.
pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .token(TOKEN.deref().to_owned())
        .proxy(String::from("localhost:3000"), true)
        .ratelimiter(None)
        .build()
});

pub type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DatabasePoolFuture = impl Future<Output = PostgresPool>;
pub static DATABASE_POOL: AsyncLazy<
    PostgresPool,
    DatabasePoolFuture,
> = AsyncLazy::new(async {
    let hartex_pgsql_url = env::var("HARTEX_NIGHTLY_PGSQL_URL").unwrap();

    let manager = PostgresConnectionManager::new_from_stringlike(hartex_pgsql_url, NoTls).unwrap();

    Pool::builder().build(manager).await.unwrap()
});

/// The bot token used for logging in to the Discord gateway and sending HTTP requests.
pub static TOKEN: Lazy<String> = Lazy::new(|| env::var("BOT_TOKEN").unwrap());
