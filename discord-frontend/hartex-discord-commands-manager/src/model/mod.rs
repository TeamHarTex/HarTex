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

use hartex_discord_core::discord::model::application::command::CommandOptionType;
use hartex_discord_core::discord::model::application::command::CommandType;
use owo_colors::OwoColorize;

pub mod command;
pub mod option;

pub trait TypeEnumExt {
    fn name(&self) -> &'static str;
}

impl TypeEnumExt for CommandType {
    fn name(&self) -> &'static str {
        match self {
            Self::ChatInput => "CHAT_INPUT",
            Self::Message => "MESSAGE",
            Self::User => "USER",
            _ => "UNKNOWN",
        }
    }
}

impl TypeEnumExt for CommandOptionType {
    fn name(&self) -> &'static str {
        match self {
            Self::Attachment => "ATTACHMENT",
            Self::Boolean => "BOOLEAN",
            Self::Channel => "CHANNEL",
            Self::Integer => "INTEGER",
            Self::Mentionable => "MENTIONABLE",
            Self::Number => "NUMBER",
            Self::Role => "ROLE",
            Self::String => "STRING",
            Self::SubCommand => "SUB_COMMAND",
            Self::SubCommandGroup => "SUB_COMMAND_GROUP",
            Self::User => "USER",
            _ => "UNKNOWN",
        }
    }
}

pub fn print_localizations(
    f: &mut Formatter<'_>,
    localizations: &HashMap<String, String>,
    depth: usize,
) -> fmt::Result {
    writeln!(f)?;

    for (locale, localization) in localizations {
        writeln!(
            f,
            "{}- {} Localization: {}",
            "    ".repeat(depth),
            locale.bright_cyan(),
            localization.bright_cyan()
        )?;
    }

    Ok(())
}
