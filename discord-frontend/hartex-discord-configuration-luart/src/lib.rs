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

//! # Configuration Lua Runtime
//!
//! This implements the Lua execution environment for obtaining the configuration for the bot when
//! needed.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_discord_configuration_models::Configuration;
use mlua::Lua;
use mlua::LuaOptions;
use mlua::Result;
use mlua::StdLib;

/// Evaluates the configuration code and returns a configuration object.
#[allow(clippy::missing_errors_doc)]
pub fn evaluate_config(config: &str) -> Result<Configuration> {
    let lua = Lua::new_with(StdLib::NONE, LuaOptions::new())?;

    let globals = lua.globals();
    globals.set("VERSION", 10)?;

    let hartexconf_table = lua.create_table()?;
    let hartexconf_colour_table = lua.create_table()?;
    let hartexconf_colour_rgb_function = lua.create_function(|_, colour: u32| Ok(colour))?;
    hartexconf_colour_table.set("rgb", hartexconf_colour_rgb_function)?;

    hartexconf_table.set("colour", hartexconf_colour_table)?;

    globals.set("hartexconf", hartexconf_table)?;

    lua.load(config).eval()
}
