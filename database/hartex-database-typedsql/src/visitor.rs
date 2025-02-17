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

use std::ops::ControlFlow;

use sqlparser::ast::Expr;
use sqlparser::ast::Value;
use sqlparser::ast::Visitor;

#[derive(Default)]
pub struct PlaceholderVisitor {
    pub(crate) placeholders: Vec<String>,
}

impl Visitor for PlaceholderVisitor {
    type Break = ();

    fn pre_visit_expr(&mut self, expr: &Expr) -> ControlFlow<Self::Break> {
        if let Expr::Value(Value::Placeholder(ph)) = expr {
            if !self.placeholders.contains(&ph[1..].to_string()) {
                self.placeholders.push(String::from(&ph[1..]));
            }
        }

        ControlFlow::Continue(())
    }
}
