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

//! # Dashboard Configuration Object

use mlua::Error;
use mlua::FromLua;
use mlua::Lua;
use mlua::Value;

/// The dashboard configuration object.
#[derive(Debug)]
pub struct Dashboard {
    /// The admins that have full edit access to the dashboard and can invite others to the
    /// dashboard.
    pub admins: Vec<String>,
    /// The editors that have access to the dashboard but cannot invite others to the
    /// dashboard.
    pub editors: Option<Vec<String>>,
    /// The viewers that only have read-only access to the dashboard and cannot change anything.
    pub viewers: Option<Vec<String>>,
}

impl<'lua> FromLua<'lua> for Dashboard {
    fn from_lua(lua_value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "Dashboard: mismatched value type, exoected table, found: {}",
                lua_value.type_name()
            )));
        };

        let admins = table.get("admins")?;
        let editors = table.get("editors")?;
        let viewers = table.get("viewers")?;

        Ok(Self {
            admins,
            editors,
            viewers,
        })
    }
}
