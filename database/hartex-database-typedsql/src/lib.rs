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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
//#![deny(warnings)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(let_chains)]

use std::collections::HashMap;
use std::path::Path;

use sqlparser::dialect::PostgreSqlDialect;

mod error;
mod query;
mod schema;

pub(crate) const POSTGRESQL_DIALECT: PostgreSqlDialect = PostgreSqlDialect {};

#[allow(clippy::missing_errors_doc)]
pub fn generate_queries_with_schemas<P>(schemas_dir: P, queries_dir: P, _: P) -> error::Result<()>
where
    P: AsRef<Path>,
{
    let schemas = schema::read_schemas(schemas_dir.as_ref())?
        .map(schema::parse_schema)
        .filter_map(Result::ok)
        .map(|schema| (schema.name.clone(), schema))
        .collect::<HashMap<_, _>>();

    let _ = query::read_queries(queries_dir.as_ref())?
        .map(|info| query::parse_query(&info, schemas.clone()))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // todo
    Ok(())
}
