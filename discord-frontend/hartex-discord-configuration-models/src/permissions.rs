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
use std::str::FromStr;

use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::marker::UserMarker;
use hartex_discord_core::discord::model::id::Id;
use itertools::Itertools;
use mlua::Error;
use mlua::FromLua;
use mlua::Lua;
use mlua::Value;
use serde::Serialize;

/// The permissions configuration object,
#[derive(Debug, Serialize)]
pub struct Permissions {
    /// Permissions map for roles.
    pub roles: HashMap<Id<RoleMarker>, u8>,
    /// Permissions map for users.
    pub users: HashMap<Id<UserMarker>, u8>,
}

impl<'lua> FromLua<'lua> for Permissions {
    fn from_lua(lua_value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "Permissions: mismatched value type, exoected table, found: {}",
                lua_value.type_name()
            )));
        };

        let Value::Table(roles) = table.get("roles") else {
            return Err(Error::RuntimeError(format!(
                "Permissions: mismatched value type, exoected table, found: {}",
                lua_value.type_name()
            )));
        };

        let Value::Table(users) = table.get("roles") else {
            return Err(Error::RuntimeError(format!(
                "Permissions: mismatched value type, exoected table, found: {}",
                lua_value.type_name()
            )));
        };

        Ok(Self {
            roles: roles
                .pairs::<String, u8>()
                .map_ok(|(key, value)| {
                    (Id::from_str(key.as_str()), value)
                })
                .collect(),
            users: users
                .pairs::<String, u8>()
                .map_ok(|(key, value)| {
                    (Id::from_str(key.as_str()), value)
                })
                .collect(),
        })
    }
}
