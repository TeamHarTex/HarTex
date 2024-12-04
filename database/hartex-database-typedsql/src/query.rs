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
use std::path::Path;

use walkdir::WalkDir;

use crate::schema::SchemaInfo;

pub(crate) struct RawQueryModuleInfo;

pub(crate) fn read_queries(
    dir: &Path,
    _: HashMap<String, SchemaInfo>,
) -> crate::error::Result<impl Iterator<Item = RawQueryModuleInfo>> {
    let _ = WalkDir::new(dir)
        .contents_first(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.path().extension().is_some())
        .filter(|entry| entry.path().extension().unwrap() == "sql");

    // todo
    Ok(std::iter::from_fn(move || Some(RawQueryModuleInfo)))
}
