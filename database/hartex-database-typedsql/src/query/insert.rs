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

use std::collections::BTreeSet;
use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use sqlparser::ast::Insert;
use sqlparser::ast::ObjectName;
use sqlparser::ast::TableObject;
use sqlparser::ast::Visit;

use crate::schema::SchemaInfo;
use crate::schema::TableInfo;
use crate::visitor::PlaceholderVisitor;

#[derive(Clone, Debug)]
pub(crate) struct InsertQueryInfo {
    pub(crate) into_table: TableInfo,
    pub(crate) placeholders: BTreeSet<String>,
}

pub(crate) fn parse_insert_query(
    insert: Insert,
    schema_infos: HashMap<String, SchemaInfo>,
) -> crate::error::Result<InsertQueryInfo> {
    let TableObject::TableName(ref name) = insert.table else {
        return Err(crate::error::Error::QueryFile(
            "table functions are not supported",
        ));
    };
    let schema_name = name
        .0
        .first()
        .ok_or(crate::error::Error::QueryFile("schema name not found"))?;
    let key = schema_name.value.to_case(Case::Snake);
    let schema_info = schema_infos
        .get(&key)
        .ok_or(crate::error::Error::QueryFile("schema not found"))?;

    let table_key = ObjectName(name.0[1..].to_vec()).to_string();
    let into_table = schema_info
        .tables
        .get(&table_key)
        .cloned()
        .ok_or(crate::error::Error::QueryFile("table not found in schema"))?;

    let mut plvisit = PlaceholderVisitor::default();
    insert.visit(&mut plvisit);

    Ok(InsertQueryInfo {
        into_table,
        placeholders: plvisit.placeholders,
    })
}
