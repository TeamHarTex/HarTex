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

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use twilight_model::application::command::CommandOption;
use twilight_model::application::command::CommandType;
use twilight_model::guild::Permissions;
pub use twilight_model::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandManagerCommand {
    pub default_member_permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub options: Vec<CommandOption>,
}

impl Display for CommandManagerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;

        Ok(())
    }
}
