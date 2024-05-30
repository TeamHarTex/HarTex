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

use mlua::FromLua;
use mlua::Lua;
use mlua::MultiValue;
use serde::Serialize;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::marker::UserMarker;
use hartex_discord_core::discord::model::id::Id;

/// The permissions configuration object,
#[derive(Debug, Serialize)]
pub struct Permissions {
    /// Permissions map for roles.
    pub roles: HashMap<Id<RoleMarker>, u8>,
    /// Permissions map for users.
    pub users: HashMap<Id<UserMarker>, u8>,
}

impl<'lua> FromLua<'lua> for Permissions {
    fn from_lua(_: MultiValue<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        todo!()
    }
}
