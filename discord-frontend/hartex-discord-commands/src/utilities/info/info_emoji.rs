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

use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::{InteractionResponse, InteractionResponseType};
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use miette::IntoDiagnostic;
use regex::Regex;

lazy_static::lazy_static! {
    static ref EMOJI_REGEX: Regex = Regex::new("<a?:[a-zA-Z0-9_]+:([0-9]{17,19})>").unwrap();
}

pub async fn execute(interaction: Interaction, option: CommandDataOption) -> miette::Result<()> {
    let CommandOptionValue::SubCommand(options) = option.value else {
        unreachable!()
    };

    let interaction_client = CLIENT.interaction(interaction.application_id);
    let locale = interaction.locale.unwrap_or_else(|| String::from("en-GB"));
    let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

    let CommandOptionValue::String(emoji) = options
        .iter()
        .find(|option| option.name.as_str() == "emoji")
        .map_or(CommandOptionValue::String(String::new()), |option| {
            option.value.clone()
        })
    else {
        unreachable!()
    };

    let emojiinfo_error_only_custom_emojis =
        localizer.utilities_plugin_emojiinfo_error_only_custom_emojis()?;
    let emojiinfo_error_only_one_emoji =
        localizer.utilities_plugin_emojiinfo_error_only_one_emoji()?;

    let Some(captures) = EMOJI_REGEX.captures(&emoji) else {
        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content(emojiinfo_error_only_custom_emojis)
                            .build(),
                    ),
                },
            )
            .await
            .into_diagnostic()?;
        return Ok(());
    };

    if captures.len() != 1 {
        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content(emojiinfo_error_only_one_emoji)
                            .build(),
                    ),
                },
            )
            .await
            .into_diagnostic()?;
        return Ok(());
    }

    Ok(())
}
