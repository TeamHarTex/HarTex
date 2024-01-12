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
use rlua::FromLuaMulti;
use rlua::MultiValue;
use rlua::Result;
use rlua::Value;

use crate::config::dashboard::Dashboard;

pub mod dashboard;

pub struct Configuration {
    pub dashboard: Dashboard,
}

impl<'lua> FromLuaMulti<'lua> for Configuration {
    fn from_lua_multi(values: MultiValue<'lua>, lua: Context<'lua>) -> Result<Self> {
        if values.is_empty() {
            return Err(Error::RuntimeError(String::from("multi value is empty")));
        }

        let Value::Table(value) = values.into_iter().next().unwrap() else {
            return Err(Error::RuntimeError(String::from("mismatched value type")));
        };

        let dashboard = Dashboard::from_lua(value.get("database")?, lua)?;

        Ok(Self { dashboard })
    }
}
