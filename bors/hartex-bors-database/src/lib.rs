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

//! # Database for Bors

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::str::FromStr;

use miette::IntoDiagnostic;
use sea_orm::DatabaseConnection;
use sea_orm::SqlxSqliteConnector;
use sea_orm_migration::prelude::MigratorTrait;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

pub mod client;
mod entity;
mod migration;
mod select_enqueued_pr;
mod select_pr;
mod select_workflow;

pub async fn initialize_database(migrate: bool) -> miette::Result<DatabaseConnection> {
    let database = SqlxSqliteConnector::from_sqlx_sqlite_pool(
        SqlitePool::connect_with(
            SqliteConnectOptions::from_str("sqlite:bors-data/data.db").into_diagnostic()?.create_if_missing(true),
        )
        .await.into_diagnostic()?,
    );

    if migrate {
        migration::Migrator::up(&database, None).await.into_diagnostic()?;
    }

    Ok(database)
}
