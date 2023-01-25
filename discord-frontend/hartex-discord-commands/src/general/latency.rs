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

use std::time::Instant;

use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_macros::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::log;
use hartex_discord_utils::CLIENT;
use hartex_eyre::eyre::Report;
use hartex_localization::create_bundle;
use hartex_localization::types::LocalizationArgs;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(name = "latency")]
pub struct Latency;

impl Command for Latency {
    async fn execute(&self, interaction: Interaction) -> hartex_eyre::Result<()> {
        let interaction_client = CLIENT.interaction(interaction.application_id);
        let bundle = create_bundle(
            interaction
                .locale
                .map_or(None, |locale| locale.parse().ok()),
            &["discord-frontend", "commands"],
        )?;
        let initial = bundle.get_term("initial-response").unwrap();
        let mut errors = Vec::new();
        let initial_response = bundle.format_pattern(&initial.value(), None, &mut errors);
        let initial_response = initial_response.trim();

        log::warn!("fluent errors occurred:");
        for error in errors {
            println!("{:?}", Report::new(error));
        }

        let initial_t = Instant::now();
        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content(&initial_response)
                            .build(),
                    ),
                },
            )
            .await?;

        let milliseconds = initial_t.elapsed().as_millis();

        let edited = bundle.get_message("edited-response").unwrap();
        let mut errors = Vec::new();
        let mut args = LocalizationArgs::new();
        args.set("latency", milliseconds);
        let edited_response = bundle.format_pattern(&edited.value().unwrap(), Some(&args), &mut errors);

        log::warn!("fluent errors occurred:");
        for error in errors {
            println!("{:?}", Report::new(error));
        }

        interaction_client
            .update_response(&interaction.token)
            .content(Some(&edited_response))?
            .await?;

        Ok(())
    }
}
