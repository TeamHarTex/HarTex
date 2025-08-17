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

use std::{env, str::FromStr, time::Duration};

use hartex_async_lazy::LazyResult;
use log::LevelFilter;
use sqlx::{
    ConnectOptions, Error, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

/// An asynchronously lazyily initialized database pool for the API Backend database.
pub static API_BACKEND: LazyResult<PgPool, Error> = LazyResult::new(|| {
    Box::pin(async {
        let hartex_pgsql_url = env::var("API_BACKEND_PGSQL_URL").unwrap();
        let options = PgConnectOptions::from_str(&hartex_pgsql_url)
            .unwrap()
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Debug, Duration::from_secs(1));
        PgPoolOptions::new().connect_with(options).await
    })
});

/// An asynchronously lazyily initialized database pool for the Configuration database.
pub static CONFIGURATION: LazyResult<PgPool, Error> = LazyResult::new(|| {
    Box::pin(async {
        let hartex_pgsql_url = env::var("CONFIGURATION_PGSQL_URL").unwrap();
        let options = PgConnectOptions::from_str(&hartex_pgsql_url)
            .unwrap()
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Debug, Duration::from_secs(1));
        PgPoolOptions::new().connect_with(options).await
    })
});

/// An asynchronously lazyily initialized database pool for the Discord Frontend database.
pub static DISCORD_FRONTEND: LazyResult<PgPool, Error> = LazyResult::new(|| {
    Box::pin(async {
        let hartex_pgsql_url = env::var("DISCORD_FRONTEND_PGSQL_URL").unwrap();
        let options = PgConnectOptions::from_str(&hartex_pgsql_url)
            .unwrap()
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Debug, Duration::from_secs(1));
        PgPoolOptions::new().connect_with(options).await
    })
});
