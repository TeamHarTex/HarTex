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
use sqlx::{query_as, Error, FromRow, PgPool, Postgres};
use sqlx::postgres::PgRow;

pub struct Idk;

impl FromRow<'_, PgRow> for Idk {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        Ok(Self {})
    }
}

async fn shitwood() {
    let mut pool = PgPool::connect("").await.unwrap();

    let thing = query_as::<Postgres, Idk>("SELECT id FROM idk ORDER BY id DESC LIMIT 5");
}
