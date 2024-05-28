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

use std::collections::HashMap;

use hartex_discord_commands::general::about::About;
use hartex_discord_commands::general::contributors::Contributors;
use hartex_discord_commands::management::role::Role;
use hartex_discord_commands::utilities::info::Info;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::traits::CommandMetadata;
use hartex_discord_core::discord::http::client::InteractionClient;
use hartex_discord_core::discord::model::application::interaction::InteractionData;
use hartex_discord_core::discord::model::gateway::payload::incoming::InteractionCreate;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use hartex_log::log;
use miette::IntoDiagnostic;
use once_cell::sync::Lazy;

use crate::errorhandler::ErrorPayload;

/// Lookup table for commands provided by the bot.
///
/// This is used for retrieving the command instance by its name such that precommand checks
/// can be executed via dynamic dispatch without the need of match arms and if guards.
pub static COMMAND_LOOKUP: Lazy<HashMap<String, Box<dyn Command + Send + Sync>>> =
    Lazy::new(|| {
        let mut map = HashMap::<String, Box<dyn Command + Send + Sync>>::new();
        map.insert(About.name(), Box::new(About));
        map.insert(Contributors.name(), Box::new(Contributors));
        map.insert(Info.name(), Box::new(Info));
        map.insert(Role.name(), Box::new(Role));
        map
    });

/// Handle an application command interaction.
#[allow(clippy::large_futures)]
pub async fn application_command(interaction_create: Box<InteractionCreate>, interaction_client: &InteractionClient<'_>) -> miette::Result<()> {
    let InteractionData::ApplicationCommand(command) = interaction_create.data.clone().unwrap()
    else {
        unreachable!("this should not be possible")
    };

    log::trace!("running interaction command {}", &command.name);

    let cloned = interaction_create.clone();

    let locale = interaction_create
        .locale
        .as_deref()
        .unwrap_or_else(|| "en-GB");
    let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

    let command = COMMAND_LOOKUP.get(&command.name).unwrap();
    let plugin = command.plugin();
    if !plugin.enabled(interaction_create.guild_id.unwrap()).await? {
        interaction_client
            .create_response(
                interaction_create.id,
                &interaction_create.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content(localizer.error_error_plugin_disabled(plugin.name())?)
                            .build(),
                    ),
                },
            )
            .await
            .into_diagnostic()?;
    }

    if let Err(error) = command
        .execute(cloned.0, interaction_client, localizer)
        .await
    {
        crate::errorhandler::handle_interaction_error(
            ErrorPayload::Miette(error),
            interaction_create,
            interaction_client,
        )
        .await;
    }

    Ok(())
}
