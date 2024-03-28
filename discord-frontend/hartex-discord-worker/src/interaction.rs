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

use miette::IntoDiagnostic;
use hartex_discord_commands::general::about::About;
use hartex_discord_commands::general::contributors::Contributors;
use hartex_discord_commands::utilities::info::Info;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::traits::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::InteractionData;
use hartex_discord_core::discord::model::gateway::payload::incoming::InteractionCreate;
use hartex_discord_core::discord::model::http::interaction::{InteractionResponse, InteractionResponseType};
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::CLIENT;
use hartex_log::log;

/// Handle an application command interaction
#[allow(clippy::large_futures)]
pub async fn application_command(interaction_create: Box<InteractionCreate>) -> miette::Result<()> {
    let InteractionData::ApplicationCommand(command) = interaction_create.data.clone().unwrap()
    else {
        unreachable!("this should not be possible")
    };

    log::trace!("running interaction command {}", &command.name);

    let cloned = interaction_create.clone();
    if let Err(error) = match command.name {
        name if name == About.name() => About.execute(cloned.0).await,
        name if name == Contributors.name() => Contributors.execute(cloned.0).await,
        name if name == Info.name() => Info.execute(cloned.0).await,
        _ => Ok(()),
    } {
        let interaction_client = CLIENT.interaction(interaction_create.application_id);
        interaction_client
            .create_response(
                interaction_create.id,
                &interaction_create.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content("This command encountered an unexpected error.")
                            .build(),
                    ),
                },
            )
            .await
            .into_diagnostic()?;

        log::warn!("unexpected error occurred for command {}: {error:?}", &command.name);
    }

    Ok(())
}
