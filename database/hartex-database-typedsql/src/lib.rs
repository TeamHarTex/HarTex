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
#![deny(warnings)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(deref_patterns)]
#![feature(let_chains)]

use std::collections::HashMap;
use std::path::Path;

use itertools::Itertools;
use sqlparser::dialect::PostgreSqlDialect;

mod codegen;
mod error;
mod query;
mod schema;
mod visitor;

pub(crate) const POSTGRESQL_DIALECT: PostgreSqlDialect = PostgreSqlDialect {};

#[allow(clippy::missing_errors_doc)]
pub fn generate_crate<P>(schemas_dir: P, queries_dir: P, target_dir: P) -> error::Result<()>
where
    P: AsRef<Path>,
{
    let schemas = schema::read_schemas(schemas_dir.as_ref())?
        .map(schema::parse_schema)
        .process_results(|iter| {
            iter.map(|schema| (schema.name.clone(), schema))
                .collect::<HashMap<_, _>>()
        })?;

    let queries = query::read_queries(queries_dir.as_ref())?
        .map(|info| query::parse_query(&info, schemas.clone()))
        .process_results(|iter| iter.collect_vec())?;

    codegen::result::generate_result_mod(&target_dir)?;

    codegen::tables::generate_table_structs_from_schemas(schemas, &target_dir)?;
    codegen::queries::generate_query_structs_from_queries(queries, &target_dir)?;

    codegen::library::generate_lib_rs(target_dir)?;

    Ok(())
}
