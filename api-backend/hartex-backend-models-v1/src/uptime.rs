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

//! # Uptime Models V1
//!
//! Models for the uptime API specifcation V1 of the backend.

use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::prelude::Row;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::Error;

/// An uptime query.
#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, Serialize)]
pub struct UptimeQuery<'a> {
    component_name: &'a str,
}

impl<'a> UptimeQuery<'a> {
    /// Create a new uptime query with the component name to search for.
    #[must_use]
    pub fn new(component_name: &'a str) -> Self {
        Self { component_name }
    }

    /// The component name to search for in this uptime query.
    #[must_use]
    pub fn component_name(&self) -> &'a str {
        self.component_name
    }
}

/// A response to an uptime query.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Deserialize, Serialize)]
pub struct UptimeResponse {
    start_timestamp: u128,
}

impl UptimeResponse {
    /// The start timestamp of the uptime entry.
    #[must_use]
    pub fn start_timestamp(&self) -> u128 {
        self.start_timestamp
    }
}

impl<'r> FromRow<'r, PgRow> for UptimeResponse {
    #[allow(clippy::cast_sign_loss)]
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let timestamp: DateTime<Utc> = row.try_get("timestamp")?;

        Ok(Self {
            start_timestamp: timestamp.timestamp() as u128,
        })
    }
}
