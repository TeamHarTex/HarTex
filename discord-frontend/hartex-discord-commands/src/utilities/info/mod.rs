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

use std::pin::Pin;

use hartex_database_queries::discord_frontend::queries::utilities_plugin_enabled::utilities_plugin_enabled;
use hartex_discord_commands_core::metadata;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::application::interaction::InteractionData;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::CLIENT;
use hartex_discord_utils::DATABASE_POOL;
use miette::IntoDiagnostic;
use tokio_postgres::GenericClient;

mod info_bot;
mod info_emoji;
mod info_role;
mod info_server;
mod info_user;

/// The `info` command declaration.
#[metadata(command_type = 1, interaction_only = true, name = "info")]
pub struct Info;

impl Command for Info {
    async fn execute(&self, interaction: Interaction) -> miette::Result<()> {
        let Some(InteractionData::ApplicationCommand(command)) = interaction.clone().data else {
            unreachable!()
        };

        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await.into_diagnostic()?;
        let client = pooled.client();
        let enabled = utilities_plugin_enabled()
            .bind(client, &interaction.guild_id.unwrap().to_string())
            .map(|json| json.0.get())
            .one()
            .await
            .map_or_else(Ok(false), |boolean| boolean.parse())
            .unwrap_or(false);

        if !enabled {
            let interaction_client = CLIENT.interaction(interaction.application_id);
            interaction_client.create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content("The `utilities` plugin is not enabled. Please enable it in the guild configuration.")
                            .build(),
                    ),
                }
            )
            .await
            .into_diagnostic()?;
        }

        let Some(subcommand) = command
            .options
            .iter()
            .find(|option| matches!(option.value, CommandOptionValue::SubCommand(_)))
        else {
            unreachable!()
        };

        match subcommand.name.as_str() {
            "bot" => info_bot::execute(interaction, subcommand.clone()).await,
            "emoji" => info_emoji::execute(interaction, subcommand.clone()).await,
            "role" => info_role::execute(interaction, subcommand.clone()).await,
            "server" => info_server::execute(interaction, subcommand.clone()).await,
            "user" => info_user::execute(interaction, subcommand.clone()).await,
            _ => unreachable!(),
        }
    }
}
