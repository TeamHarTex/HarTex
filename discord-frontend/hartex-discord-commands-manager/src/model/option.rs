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
use std::fmt::Formatter;

use hartex_discord_core::discord::model::application::command::CommandOptionChoice;
use hartex_discord_core::discord::model::application::command::CommandOptionType;
use hartex_discord_core::discord::model::application::command::CommandOptionValue;
use hartex_discord_core::discord::model::channel::ChannelType;
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde::Serialize;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandManagerCommandOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<CommandOptionChoice>>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
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
            "{}- {}",
            "    ".repeat(depth),
            "Command Option Name Localizations: ".bold(),
        )?;

        Ok(())
    }
}
