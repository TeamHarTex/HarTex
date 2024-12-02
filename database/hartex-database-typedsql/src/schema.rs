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

use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub(crate) struct SchemaInfo;

pub(crate) struct RawSchemaInfo {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) contents: String,
}

pub(crate) fn read_schemas(dir: &Path) -> crate::error::Result<Vec<RawSchemaInfo>> {
    let mut vec = Vec::new();

    for result in fs::read_dir(dir)? {
        let entry = result?;
        let path = entry.path();
        if !path.extension().map(|s| s == "sql").unwrap_or_default() {
            continue;
        }

        let name = path.file_stem().expect("is a file").to_str().expect("valid UTF-8").to_string();
        let contents = fs::read_to_string(&path)?;

        vec.push(RawSchemaInfo {
            path,
            name,
            contents,
        })
    }

    Ok(vec)
}

pub(crate) fn parse_schema(_: RawSchemaInfo) -> crate::error::Result<SchemaInfo> {
    todo!()
}
