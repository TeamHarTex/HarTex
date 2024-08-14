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

//! # Event Bitflags

use bitflags::Flags;
use hartex_bitflags_utils::FlagsExt;
use mlua::Error;
use mlua::FromLua;
use mlua::Lua;
use mlua::Value;
use serde::Serialize;

bitflags::bitflags! {
    #[derive(Debug, Serialize)]
    pub struct EventFlags: u128 {
        const MESSAGE_DELETED = 1;
        const MESSAGE_UPDATED = 1 << 1;
    }
}

impl<'lua> FromLua<'lua> for EventFlags {
    fn from_lua(lua_value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "EventFlags: mismatched value type, expected table, found: {}",
                lua_value.type_name()
            )));
        };

        let known = EventFlags::FLAGS
            .iter()
            .map(|flag| flag.name().to_string())
            .collect::<Vec<_>>();

        let flags = table
            .sequence_values::<String>()
            .map(|flag| {
                if known.contains(&flag.clone()?) {
                    flag
                } else {
                    Err(Error::RuntimeError(format!(
                        "EventFlags: unknown flag {}",
                        flag?
                    )))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::from_names(flags))
    }
}
