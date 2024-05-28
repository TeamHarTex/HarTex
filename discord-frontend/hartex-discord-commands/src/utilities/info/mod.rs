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

//! # The Info Command

use async_trait::async_trait;
use hartex_discord_commands_core::command;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_core::discord::http::client::InteractionClient;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::application::interaction::InteractionData;
use hartex_localization_core::Localizer;

use crate::utilities::Utilities;

mod info_bot;
mod info_emoji;
mod info_role;
mod info_server;
mod info_user;

/// The `info` command declaration.
#[command(name = "info", plugin = Utilities)]
pub struct Info;

#[async_trait]
impl Command for Info {
    async fn execute(
        &self,
        interaction: Interaction,
        interaction_client: &InteractionClient<'_>,
        localizer: Localizer<'_>,
    ) -> miette::Result<()> {
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
            "bot" => {
                info_bot::execute(
                    interaction,
                    interaction_client,
                    subcommand.clone(),
                    localizer,
                )
                .await
            }
            "emoji" => {
                info_emoji::execute(
                    interaction,
                    interaction_client,
                    subcommand.clone(),
                    localizer,
                )
                .await
            }
            "role" => {
                info_role::execute(
                    interaction,
                    interaction_client,
                    subcommand.clone(),
                    localizer,
                )
                .await
            }
            "server" => {
                info_server::execute(
                    interaction,
                    interaction_client,
                    subcommand.clone(),
                    localizer,
                )
                .await
            }
            "user" => {
                info_user::execute(
                    interaction,
                    interaction_client,
                    subcommand.clone(),
                    localizer,
                )
                .await
            }
            _ => unreachable!(),
        }
    }
}
