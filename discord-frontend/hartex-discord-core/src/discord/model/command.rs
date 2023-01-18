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

use owo_colors::OwoColorize;
use serde::Deserialize;
use serde::Serialize;
use twilight_model::application::command::CommandType;
use twilight_model::guild::Permissions;

use super::option::CommandManagerCommandOption;
use crate::discord::extensions::CommandTypeExt;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandManagerCommand {
    pub default_member_permissions: Option<Permissions>,
    #[deprecated(note = "use default_member_permissions instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    pub options: Vec<CommandManagerCommandOption>,
}

impl Display for CommandManagerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "{}{}", "Command Name: ".bold(), self.name.bright_cyan())?;

        write!(f, "{}", "Command Name Localizations: ".bold())?;
        if self.name_localizations.is_some() {
            for (locale, localization) in self.name_localizations.as_ref().unwrap() {
                writeln!(f)?;
                writeln!(
                    f,
                    "    - {} Localization: {}",
                    locale.bright_cyan(),
                    localization.bright_cyan()
                )?;
            }
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        write!(f, "{}", "Command Description: ".bold())?;
        if self.description.is_some() {
            writeln!(f, "{}", self.description.as_ref().unwrap().bright_cyan())?;
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        write!(f, "{}", "Command Description Localizations: ".bold())?;
        if self.description_localizations.is_some() {
            for (locale, localization) in self.description_localizations.as_ref().unwrap() {
                writeln!(f)?;
                writeln!(
                    f,
                    "    - {} localization: {}",
                    locale.bright_cyan(),
                    localization.bright_cyan()
                )?;
            }
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        writeln!(
            f,
            "{}{}",
            "Command Type: ".bold(),
            self.kind.name().bright_cyan()
        )?;

        writeln!(
            f,
            "{}{}",
            "Command Visibility in Direct Messages: ".bold(),
            self.dm_permission
                .map_or("Visible", |permission| if permission {
                    "Visible"
                } else {
                    "Invisible"
                })
                .bright_cyan()
        )?;

        #[allow(deprecated)]
        writeln!(
            f,
            "{}{}",
            "Command Enabled By Default (deprecated): ".yellow().bold(),
            self.default_permissions
                .map_or("Yes", |permission| if permission { "Yes" } else { "No" })
                .bright_cyan()
        )?;

        write!(f, "{}", "Command Age Restricted: ".bold())?;
        if self.nsfw.is_some() {
            writeln!(
                f,
                "{}",
                if self.nsfw.unwrap() { "Yes" } else { "No" }.bright_cyan()
            )?;
        } else {
            writeln!(f, "{}", "Unspecified".truecolor(107, 107, 107))?;
        }

        write!(f, "{}", "Command Default Member Permissions: ".bold())?;
        if self.default_member_permissions.is_some() {
            let permissions_string = format!("{:?}", self.default_member_permissions.unwrap());
            writeln!(f)?;

            for permission in permissions_string.split(" | ") {
                writeln!(f, "    - {}", permission.bright_cyan())?;
            }
        } else {
            writeln!(f, "{}", "Unspecified".truecolor(107, 107, 107))?;
        }

        write!(f, "{}", "Command Options: ".bold())?;
        if self.options.is_empty() {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        } else {
            writeln!(f)?;

            for option in &self.options {
                writeln!(f, "{option}")?;
            }
        }

        Ok(())
    }
}
