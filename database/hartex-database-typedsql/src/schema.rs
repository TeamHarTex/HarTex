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

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use sqlparser::ast::Statement;
use sqlparser::parser::Parser;
use crate::error::Error;
use crate::POSTGRESQL_DIALECT;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct ColumnInfo {
    pub(crate) name: String,
    pub(crate) coltype: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct SchemaInfo {
    pub(crate) name: String,
    pub(crate) tables: Vec<TableInfo>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct TableInfo {
    pub(crate) name: String,
    pub(crate) columns: Vec<ColumnInfo>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct RawSchemaInfo {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) contents: String,
}

#[allow(clippy::missing_errors_doc)]
pub(crate) fn read_schemas(
    dir: &Path,
) -> crate::error::Result<impl Iterator<Item = RawSchemaInfo>> {
    Ok(fs::read_dir(dir)?.filter_map(|result| {
        let Ok(entry) = result else {
            return None;
        };
        let path = entry.path();
        if path.extension().is_none_or(|s| s != "sql") {
            return None;
        }

        let name = path
            .file_stem()
            .expect("is a file")
            .to_str()
            .expect("valid UTF-8")
            .to_string();
        let Ok(contents) = fs::read_to_string(&path) else {
            return None;
        };

        Some(RawSchemaInfo {
            path,
            name,
            contents,
        })
    }))
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn parse_schema(schema_info: RawSchemaInfo) -> crate::error::Result<SchemaInfo> {
    let statements = Parser::parse_sql(&POSTGRESQL_DIALECT, schema_info.contents.as_str())?;

    panic!("{statements:?}");
}
