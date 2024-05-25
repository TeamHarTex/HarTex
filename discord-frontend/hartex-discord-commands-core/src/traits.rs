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

use async_trait::async_trait;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::Id;

/// The command metadata trait, specifying the various information about a command.
pub trait CommandMetadata {
    type Plugin: Plugin;

    /// The type of the command.
    #[deprecated(since = "0.11.0")]
    fn command_type(&self) -> u8 {
        0
    }

    /// Whether the command is only available in the form of an interaction.
    #[deprecated(since = "0.11.0")]
    fn interaction_only(&self) -> bool {
        true
    }

    /// The name of the command.
    fn name(&self) -> String;

    /// The plugin the command belongs to.
    fn plugin(&self) -> Self::Plugin;
}

/// The command trait, contains callbacks that are to be run before or when an interaction command
/// is handled.
#[async_trait]
pub trait Command: CommandMetadata {
    /// Executes the command.
    async fn execute(&self, interaction: Interaction) -> miette::Result<()>;
}

/// The plugin metadata data specifying information about a plugin.
pub trait PluginMetadata {
    /// The name of the plugin.
    fn name(&self) -> String;
}

/// The plugin trait, contains callbacks that are to be run before or when an interaction command is
/// handled.
#[async_trait]
pub trait Plugin: PluginMetadata {
    /// Whether a given plugin is enabled.
    async fn enabled(&self, guild_id: Id<GuildMarker>) -> bool;
}
