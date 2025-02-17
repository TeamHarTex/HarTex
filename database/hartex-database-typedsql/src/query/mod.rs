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

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use itertools::Itertools;
use sqlparser::ast::DataType;
use sqlparser::ast::Query;
use sqlparser::ast::SetExpr;
use sqlparser::ast::Statement;
use sqlparser::parser::Parser;
use walkdir::WalkDir;

use crate::POSTGRESQL_DIALECT;
use crate::schema::SchemaInfo;

pub(crate) mod insert;
pub(crate) mod select;
mod types;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) enum QueryInfoInner {
    Insert(insert::InsertQueryInfo),
    Select(select::SelectQueryInfo),
}

#[derive(Clone, Debug)]
pub(crate) struct QueryInfo {
    pub(crate) path: String,
    pub(crate) raw: Statement,
    pub(crate) inner: QueryInfoInner,
    pub(crate) extra_placeholder_tys: HashMap<String, DataType>,
}

#[derive(Clone, Debug)]
pub(crate) struct RawQueryInfo {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) contents: String,
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn read_queries(dir: &Path) -> crate::error::Result<impl Iterator<Item = RawQueryInfo>> {
    Ok(WalkDir::new(dir)
        .contents_first(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.path().extension().is_some())
        .filter(|entry| entry.path().extension().unwrap() == "sql")
        .filter_map(|entry| {
            Some(RawQueryInfo {
                path: entry.clone().into_path(),
                name: entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                contents: fs::read_to_string(entry.path()).ok()?,
            })
        }))
}

pub(crate) fn parse_query(
    query_info: &RawQueryInfo,
    schema_map: &BTreeMap<String, SchemaInfo>,
) -> crate::error::Result<(String, QueryInfo)> {
    let statement = Parser::parse_sql(&POSTGRESQL_DIALECT, &query_info.contents)?
        .first()
        .cloned()
        .ok_or(crate::error::Error::QueryFile(
            "no query found in query file",
        ))?;

    let mut path = query_info.path.clone();
    path.pop();
    let parent = path.components().next_back().unwrap().as_os_str();

    let inner = match statement.clone() {
        Statement::Insert(insert) => {
            QueryInfoInner::Insert(insert::parse_insert_query(&insert, schema_map)?)
        }
        Statement::Query(
            deref!(Query {
                body: deref!(SetExpr::Select(deref!(ref select))),
                ..
            }),
        ) => QueryInfoInner::Select(select::parse_select_query(select, schema_map)?),
        _ => return Err(crate::error::Error::QueryFile("unsupported query type")),
    };

    let extra_placeholder_tys = query_info
        .contents
        .lines()
        .filter(|line| line.starts_with("-- "))
        .map(|line| line[3..].to_string())
        .map(|spec| {
            let vec = spec.split(':').collect_vec();
            (
                vec[0].to_string(),
                types::str_to_sql_data_type(vec[1]).unwrap(),
            )
        })
        .collect();

    Ok((query_info.name.clone(), QueryInfo {
        raw: statement,
        path: parent.to_string_lossy().to_string(),
        inner,
        extra_placeholder_tys,
    }))
}
