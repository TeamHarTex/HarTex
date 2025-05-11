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

// THIS IS A TEMPORARY FILE

use sqlx::Postgres;
use sqlx::postgres::{PgArguments, PgPool};
use sqlx::query::{Query, QueryAs};

// INSERT INTO things
pub struct IdkInsert<'a> {
    pool: &'a mut PgPool,
    query: Query<'a, Postgres, PgArguments>,
}

// SELECT things
pub struct IdkSelect<'a> {
    pool: &'a mut PgPool,
    query: QueryAs<'a, Postgres, (), PgArguments>,
}

impl<'a> IdkInsert<'a> {
    pub fn new(pool: &'a mut PgPool) -> Self {
        Self {
            pool,
            query: sqlx::query(""),
        }
    }

    #[must_use = "Queries must be executed after construction"]
    pub fn bind(self) -> Self {
        self
    }
}

impl<'a> IdkSelect<'a> {
    pub fn new(pool: &'a mut PgPool) -> Self {
        Self {
            pool,
            query: sqlx::query_as(""),
        }
    }

    #[must_use = "Queries must be executed after construction"]
    pub fn bind(self) -> Self {
        self
    }
}
