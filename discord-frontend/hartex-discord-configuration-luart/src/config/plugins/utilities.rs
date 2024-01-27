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

use rlua::Context;
use rlua::Error;
use rlua::FromLua;
use rlua::Value;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct UtilitiesPlugin {
    pub enabled: bool,
}

impl<'lua> FromLua<'lua> for UtilitiesPlugin {
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "Dashboard: mismatched value type, exoected table, found: {}",
                lua_value.type_name()
            )));
        };

        let enabled = table.get("enabled")?;

        Ok(Self { enabled })
    }
}
