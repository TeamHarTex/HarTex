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

use std::env;

use miette::IntoDiagnostic;
use hartex_log::log;
use tokio_postgres::NoTls;

refinery::embed_migrations!();

#[tokio::main]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    log::trace!("establishing database connection");
    let url = env::var("API_PGSQL_URL").unwrap();
    let (mut client, connection) =
        tokio_postgres::connect(&url, NoTls).await.into_diagnostic()?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            log::error!("postgres connection error: {error}");
        }
    });

    migrations::runner().run_async(&mut client).await.into_diagnostic()?;

    Ok(())
}
