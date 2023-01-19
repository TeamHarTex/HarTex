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

use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use hartex_discord_core::discord::model::application::command::CommandOptionType;
use hartex_discord_core::discord::model::application::command::CommandOptionValue;
use hartex_discord_core::discord::model::channel::ChannelType;
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde::Serialize;

use super::TypeEnumExt;

pub mod choice;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandManagerCommandOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<choice::CommandManagerCommandOptionChoice>>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<CommandOptionValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<CommandOptionValue>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CommandManagerCommandOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CommandManagerCommandOption {
    pub fn display(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        writeln!(
            f,
            "{}- {}{}",
            "    ".repeat(depth),
            "Command Option Name: ".bold(),
            self.name.bright_cyan()
        )?;

        write!(
            f,
            "{}  {}",
            "    ".repeat(depth),
            "Command Option Name Localizations: ".bold(),
        )?;
        if self.name_localizations.is_some() {
            super::print_localizations(f, self.name_localizations.as_ref().unwrap(), depth + 1)?;
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        writeln!(
            f,
            "{}- {}{}",
            "    ".repeat(depth),
            "Command Option Description: ".bold(),
            self.description.bright_cyan()
        )?;

        write!(
            f,
            "{}  {}",
            "    ".repeat(depth),
            "Command Option Description Localizations: ".bold(),
        )?;
        if self.description_localizations.is_some() {
            super::print_localizations(
                f,
                self.description_localizations.as_ref().unwrap(),
                depth + 1,
            )?;
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        writeln!(
            f,
            "{}  {}{}",
            "    ".repeat(depth),
            "Command Option Type: ".bold(),
            self.kind.name().bright_cyan()
        )?;

        writeln!(
            f,
            "{}  {}{}",
            "    ".repeat(depth),
            "Command Option Required: ".bold(),
            self.required
                .map_or("No", |required| if required { "Yes" } else { "No" })
                .bright_cyan()
        )?;

        writeln!(
            f,
            "{}  {}{}",
            "    ".repeat(depth),
            "Command Option Autocomplete Enabled: ".bold(),
            self.autocomplete
                .map_or("No", |required| if required { "Yes" } else { "No" })
                .bright_cyan()
        )?;

        write!(
            f,
            "{}  {}",
            "    ".repeat(depth),
            "Command Option Channel Types: ".bold()
        )?;
        if self.channel_types.is_some() {
            writeln!(f)?;

            for channel_type in self.channel_types.as_ref().unwrap() {
                writeln!(
                    f,
                    "{}- {}",
                    "    ".repeat(depth + 1),
                    channel_type.name().bright_cyan()
                )?;
            }
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        writeln!(
            f,
            "{}  {}{}",
            "    ".repeat(depth),
            "Command Option Minimum Allowed Length: ".bold(),
            self.min_length
                .map_or(String::from("Unspecified"), |min_length| min_length
                    .to_string())
                .bright_cyan()
        )?;
        writeln!(
            f,
            "{}  {}{}",
            "    ".repeat(depth),
            "Command Option Maximum Allowed Length: ".bold(),
            self.max_length
                .map_or(String::from("Unspecified"), |max_length| max_length
                    .to_string())
                .bright_cyan()
        )?;

        write!(
            f,
            "{}  {}",
            "    ".repeat(depth),
            "Command Option Choices: ".bold(),
        )?;
        if self.choices.is_some() {
            writeln!(f)?;

            for choice in self.choices.as_ref().unwrap() {
                choice.display(f, depth + 1)?;
            }
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        Ok(())
    }
}
