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

use rlua::Lua;
use rlua::Result;
use rlua::StdLib;

pub fn evaluate_config(config: &str) -> Result<()> {
    Lua::new_with(StdLib::BASE)
        .context(|ctx| {
            let globals = ctx.globals();
            globals.set("VERSION", 10)?;

            let hartexconf_table = ctx.create_table()?;
            let hartexconf_colour_table = ctx.create_table()?;
            let hartexconf_colour_rgb_function = ctx.create_function(|_, colour: u32| {
                Ok(colour)
            })?;
            hartexconf_colour_table.set("rgb", hartexconf_colour_rgb_function)?;

            hartexconf_table.set("colour", hartexconf_colour_table)?;

            globals.set("hartexconf", hartexconf_table)?;

            ctx.load(config).eval()
        })?
}
