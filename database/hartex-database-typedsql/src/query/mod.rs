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

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use pg_query::protobuf::node::Node;
use walkdir::WalkDir;

use crate::error::Error;
use crate::schema::SchemaInfo;

mod select;

pub(crate) struct QueryInfo;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct RawQueryInfo {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) contents: String,
}

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

#[allow(dead_code)]
pub(crate) fn parse_query(
    query_info: RawQueryInfo,
    schema_map: HashMap<String, SchemaInfo>,
) -> crate::error::Result<QueryInfo> {
    let result = pg_query::parse(query_info.contents.as_str())?;
    let stmt = result
        .protobuf
        .stmts
        .first()
        .cloned()
        .ok_or(Error::QueryFile("expected at least one query"))?
        .stmt
        .ok_or(Error::QueryFile("unexpected empty node"))?
        .node
        .ok_or(Error::QueryFile("unexpected empty inner node"))?;

    match stmt {
        // todo: add more branches
        Node::SelectStmt(stmt) => select::parse_select_query(stmt.as_ref().clone(), schema_map),
        _ => Err(Error::QueryFile("unexpected statement type")),
    }
}
