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

use crate::schema::SchemaInfo;

mod tables;

pub(crate) fn generate_table_structs_from_schemas<P>(
    schemas: HashMap<String, SchemaInfo>,
    root_path: P,
) -> crate::error::Result<()>
where
    P: AsRef<Path>,
{
    let tables_dir = root_path.as_ref().join("tables");
    fs::create_dir_all(&tables_dir)?;

    let _ = schemas.into_iter().map(tables::generate_table_structs_from_schema).collect::<Vec<_>>();

    todo!()
}
