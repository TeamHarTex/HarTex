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

use convert_case::Case;
use convert_case::Casing;
use sqlparser::ast::Expr;
use sqlparser::ast::ObjectName;
use sqlparser::ast::Query;
use sqlparser::ast::Select;
use sqlparser::ast::SelectItem;
use sqlparser::ast::SetExpr;
use sqlparser::ast::TableFactor;
use sqlparser::ast::Value;
use sqlparser::ast::Visit;

use crate::schema::SchemaInfo;
use crate::schema::TableInfo;
use crate::visitor::PlaceholderVisitor;

#[derive(Clone, Debug)]
pub(crate) enum SelectWhat {
    Boolean(bool),
    Everything,
    Exists(SelectQueryInfo),
}

#[derive(Clone, Debug)]
pub(crate) struct SelectQueryInfo {
    pub(crate) what: Box<SelectWhat>,
    pub(crate) from: Option<TableInfo>,
    pub(crate) placeholders: Vec<String>,
}

pub(crate) fn parse_select_query(
    select: Select,
    schema_infos: BTreeMap<String, SchemaInfo>,
) -> crate::error::Result<SelectQueryInfo> {
    let what = match select.projection.first() {
        Some(SelectItem::UnnamedExpr(Expr::Exists {
            subquery:
                deref!(Query {
                    body: deref!(SetExpr::Select(deref!(select_inner))),
                    ..
                }),
            ..
        })) => SelectWhat::Exists(parse_select_query(
            select_inner.clone(),
            schema_infos.clone(),
        )?),
        Some(SelectItem::UnnamedExpr(Expr::Value(Value::Boolean(boolean)))) => {
            SelectWhat::Boolean(*boolean)
        }
        Some(SelectItem::Wildcard(_)) if select.projection.len() == 1 => SelectWhat::Everything,
        _ => {
            return Err(crate::error::Error::QueryFile(
                "unsupported selection projection",
            ));
        }
    };

    let from = if let Some(tablewj) = select.from.first()
        && let TableFactor::Table { ref name, .. } = tablewj.relation
    {
        let schema_name = name
            .0
            .first()
            .ok_or(crate::error::Error::QueryFile("schema name not found"))?;
        let key = schema_name.value.to_case(Case::Snake);
        let schema_info = schema_infos
            .get(&key)
            .ok_or(crate::error::Error::QueryFile("schema not found"))?;

        let table_key = ObjectName(name.0[1..].to_vec()).to_string();
        schema_info.tables.get(&table_key).cloned()
    } else {
        None
    };

    let mut plvisit = PlaceholderVisitor::default();
    select.visit(&mut plvisit);

    Ok(SelectQueryInfo {
        what: Box::new(what),
        from,
        placeholders: plvisit.placeholders,
    })
}
