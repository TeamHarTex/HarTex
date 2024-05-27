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

//! # Appearance Configuration Object

use mlua::Error;
use mlua::FromLua;
use mlua::Lua;
use mlua::Value;
use serde::Serialize;

/// The appearance configuration object.
#[derive(Debug, Serialize)]
pub struct Appearance {
    /// The role colour of the bot.
    pub colour: Option<u32>,
    /// The nickname of the bot in the server.
    pub nickname: Option<String>,
}

impl<'lua> FromLua<'lua> for Appearance {
    fn from_lua(lua_value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "Appearance: mismatched value type, exoected table, found: {}",
                lua_value.type_name()
            )));
        };

        let colour = table.get("colour")?;
        let nickname = table.get("nickname")?;

        Ok(Self { colour, nickname })
    }
}
