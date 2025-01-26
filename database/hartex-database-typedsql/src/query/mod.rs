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

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use walkdir::WalkDir;

use crate::error::Error;
use crate::schema::SchemaInfo;

mod select;

#[derive(Clone, Debug)]
pub(crate) enum QueryInfo {
    Select(select::SelectQueryInfo),
}

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

pub(crate) fn parse_query(
    query_info: &RawQueryInfo,
    schema_map: HashMap<String, SchemaInfo>,
) -> crate::error::Result<QueryInfo> {
    Err(Error::QueryFile(""))
}
