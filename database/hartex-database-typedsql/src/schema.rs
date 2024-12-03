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

use pg_query::protobuf::node::Node;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct ColumnInfo {
    pub(crate) name: String,
    pub(crate) coltype: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct SchemaInfo {
    pub(crate) name: String,
    pub(crate) tables: Vec<TableInfo>,
}

#[allow(dead_code)]
#[derive(Debug)]
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
pub(crate) fn read_schemas(dir: &Path) -> crate::error::Result<Vec<RawSchemaInfo>> {
    let mut vec = Vec::new();

    for result in fs::read_dir(dir)? {
        let entry = result?;
        let path = entry.path();
        if path.extension().is_none_or(|s| s != "sql") {
            continue;
        }

        let name = path
            .file_stem()
            .expect("is a file")
            .to_str()
            .expect("valid UTF-8")
            .to_string();
        let contents = fs::read_to_string(&path)?;

        vec.push(RawSchemaInfo {
            path,
            name,
            contents,
        });
    }

    Ok(vec)
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn parse_schema(schema_info: RawSchemaInfo) -> crate::error::Result<SchemaInfo> {
    let result = pg_query::parse(schema_info.contents.as_str())?;
    let statements = result.protobuf.stmts;
    let tables = statements
        .into_iter()
        .filter_map(|statement| {
            if let Some(Node::CreateStmt(create)) = statement.clone().stmt?.node {
                Some(create)
            } else {
                None
            }
        })
        .filter_map(|create| {
            create
                .relation
                .and_then(|relation| Some((relation, create.table_elts)))
        })
        .map(|(relation, nodes)| {
            (
                relation,
                nodes
                    .into_iter()
                    .filter_map(|node| {
                        if let Some(Node::ColumnDef(def)) = node.node {
                            Some(def)
                        } else {
                            None
                        }
                    })
                    .filter(|def| def.type_name.is_some())
                    .map(|def| (def.colname, def.type_name.unwrap()))
                    .map(|(name, coltype)| {
                        let Node::String(string) =
                            coltype.clone().names.last().unwrap().clone().node.unwrap()
                        else {
                            unreachable!()
                        };

                        ColumnInfo {
                            name,
                            coltype: string.sval,
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(relation, columns)| TableInfo {
            name: relation.relname,
            columns,
        })
        .collect::<Vec<_>>();

    Ok(
        SchemaInfo {
            name: schema_info.name,
            tables,
        }
    )
}
