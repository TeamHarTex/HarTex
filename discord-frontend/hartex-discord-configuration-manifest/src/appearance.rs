/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

use hcl::eval::FuncArgs;
use hcl::Number;
use hcl::Value;
use serde::Deserialize;
use serde::Serialize;

/// Appearance of HarTex in the server.
#[derive(Deserialize, Serialize)]
pub struct Appearance {
    /// Nickname of the bot user in the server.
    pub nickname: Option<String>,
    /// The role colour of the bot's integration role.
    pub role_colour: Option<u32>,
}

pub(crate) fn hcl_rgb_function(args: FuncArgs) -> Result<Value, String> {
    let r = args[0].as_number().unwrap();
    let g = args[1].as_number().unwrap();
    let b = args[2].as_number().unwrap();

    Ok(Value::Number(Number::from(u32::from_be_bytes([
        r.as_u64().unwrap() as u8,
        g.as_u64().unwrap(),
        b.as_u64().unwrap(),
    ]))))
}
