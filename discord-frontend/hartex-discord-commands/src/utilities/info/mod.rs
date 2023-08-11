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

use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::application::interaction::InteractionData;

mod info_bot;
mod info_role;
mod info_server;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(minimum_level = 0)]
#[metadata(name = "info")]
pub struct Info;

impl Command for Info {
    async fn execute(&self, interaction: Interaction) -> miette::Result<()> {
        let Some(InteractionData::ApplicationCommand(command)) = interaction.clone().data else {
            unreachable!()
        };

        let Some(subcommand) = command
            .options
            .iter()
            .find(|option| matches!(option.value, CommandOptionValue::SubCommand(_)))
        else {
            unreachable!()
        };

        match subcommand.name.as_str() {
            "bot" => info_bot::execute(interaction, subcommand.clone()).await,
            "role" => info_role::execute(interaction, subcommand.clone()).await,
            "server" => info_server::execute(interaction, subcommand.clone()).await,
            _ => unreachable!(),
        }
    }
}
