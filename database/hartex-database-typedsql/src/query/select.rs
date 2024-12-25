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

use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::node::Node;

use crate::schema::SchemaInfo;

#[derive(Clone, Debug)]
pub(crate) enum SelectTarget {
    Everything,
}

#[derive(Clone, Debug)]
pub(crate) struct SelectQueryInfo {
    pub(crate) target: SelectTarget,
}

pub(crate) fn parse_select_query(
    stmt: SelectStmt,
    _: HashMap<String, SchemaInfo>,
) -> crate::error::Result<super::QueryInfo> {
    let target_column_fields = stmt
        .target_list
        .into_iter()
        .filter_map(|node| node.node)
        .filter_map(|node| {
            if let Node::ResTarget(res_target) = node {
                Some(res_target)
            } else {
                None
            }
        })
        .filter_map(|res_target| res_target.val)
        .filter_map(|node| node.node)
        .filter_map(|node| {
            if let Node::ColumnRef(column_ref) = node {
                Some(column_ref)
            } else {
                None
            }
        })
        .map(|node| {
            node.fields
                .into_iter()
                .filter_map(|node| node.node)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let target = parse_select_target(target_column_fields)?;

    Ok(super::QueryInfo::Select(SelectQueryInfo { target }))
}

fn parse_select_target(fields: Vec<Vec<Node>>) -> crate::error::Result<SelectTarget> {
    let _ = fields.first().ok_or(crate::error::Error::QueryFile(
        "expected at least one node in select target",
    ))?;

    todo!()
}
