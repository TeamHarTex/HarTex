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

//! # Modlog Logger Configuration Object

use mlua::{Error, FromLua, Lua, Value};
use serde::Serialize;

/// The modlog logger configuration object.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize)]
pub struct ModlogLogger {
    /// The channel ID associated with this logger.
    pub channel: String,
    /// The events this logger listens to.
    pub events: Vec<String>,
    /// The format the logger is configured to use.
    pub format: ModlogFormat,
}

impl<'lua> FromLua<'lua> for ModlogLogger {
    fn from_lua(lua_value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "ModlogPlugin: mismatched value type, expected table, found: {}",
                lua_value.type_name()
            )));
        };

        let channel = table.get("channel")?;
        let events = table.get("events")?;
        let format = table.get("format")?;

        Ok(Self {
            channel,
            events,
            format,
        })
    }
}

/// Configures modlog formats.
#[derive(Debug, Serialize)]
pub enum ModlogFormat {
    /// Default, text-only format.
    Default,
    /// Embeds are used.
    Pretty,
}

impl Default for ModlogFormat {
    fn default() -> Self {
        Self::Default
    }
}

impl<'lua> FromLua<'lua> for ModlogFormat {
    fn from_lua(_: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        todo!()
    }
}
