/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use base::discord::model::application::command::{CommandOption, CommandType};
use base::discord::model::guild::Permissions;
use base::discord::model::id::marker::ApplicationMarker;
use base::discord::model::id::Id;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct HarTexCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    pub r#type: CommandType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(default)]
    pub options: Option<Vec<CommandOption>>,
}

impl Default for HarTexCommand {
    fn default() -> Self {
        Self {
            application_id: None,
            default_member_permissions: None,
            dm_permission: None,
            description: None,
            description_localizations: None,
            r#type: CommandType::ChatInput,
            name: String::default(),
            name_localizations: None,
            options: None,
        }
    }
}
