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

//! # The Info Emoji Subcommand
//!
//! This command returns information of a custom Discord emoji.

use std::str::FromStr;

use hartex_discord_core::discord::http::client::InteractionClient;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::id::marker::EmojiMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_entitycache_core::error::CacheError;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::emoji::CachedEmojiRepository;
use hartex_discord_utils::commands::CommandDataOptionExt;
use hartex_discord_utils::commands::CommandDataOptionsExt;
use hartex_discord_utils::interaction::embed_response;
use hartex_discord_utils::interaction::ephemeral_error_response;
use hartex_discord_utils::localizable::Localizable;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::postgres::PostgresErrorExt;
use hartex_localization_core::Localizer;
use miette::IntoDiagnostic;
use regex::Regex;
use tokio_postgres::error::SqlState;

lazy_static::lazy_static! {
    /// The regex for looking for a Discord emoji in the command input.
    static ref EMOJI_REGEX: Regex = Regex::new("<a?:[a-zA-Z0-9_]+:([0-9]{17,19})>").unwrap();
}

/// Executes the `info emoji` command.
#[allow(clippy::too_many_lines)]
pub async fn execute(
    interaction: Interaction,
    interaction_client: &InteractionClient<'_>,
    option: CommandDataOption,
    localizer: Localizer<'_>,
) -> miette::Result<()> {
    let options = option.assume_subcommand();

    let langid_locale = interaction
        .locale
        .clone()
        .and_then(|locale| locale.parse().ok());

    let emoji = options.string_value_of("emoji");

    let emojiinfo_error_only_custom_emojis =
        localizer.utilities_plugin_emojiinfo_error_only_custom_emojis()?;
    let emojiinfo_error_only_one_emoji =
        localizer.utilities_plugin_emojiinfo_error_only_one_emoji()?;
    let emojiinfo_error_unknown_emoji =
        localizer.utilities_plugin_emojiinfo_error_unknown_emoji()?;

    let Some(captures) = EMOJI_REGEX.captures(&emoji) else {
        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &ephemeral_error_response(emojiinfo_error_only_custom_emojis),
            )
            .await
            .into_diagnostic()?;

        return Ok(());
    };

    if captures.len() > 2 {
        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &ephemeral_error_response(emojiinfo_error_only_one_emoji),
            )
            .await
            .into_diagnostic()?;

        return Ok(());
    }

    let id = captures.get(1).unwrap().as_str();
    let emoji_id = Id::<EmojiMarker>::from_str(id).unwrap();

    let result = CachedEmojiRepository.get(emoji_id).await;
    let emoji = match result {
        Ok(emoji) => emoji,
        Err(CacheError::Postgres(postgres_error)) if postgres_error.is(SqlState::NO_DATA) => {
            interaction_client
                .create_response(
                    interaction.id,
                    &interaction.token,
                    &ephemeral_error_response(emojiinfo_error_unknown_emoji),
                )
                .await
                .into_diagnostic()?;

            return Ok(());
        }
        error => error.into_diagnostic()?,
    };

    let emojiinfo_embed_generalinfo_field_name =
        localizer.utilities_plugin_emojiinfo_embed_generalinfo_field_name()?;
    let emojiinfo_embed_generalinfo_id_subfield_name =
        localizer.utilities_plugin_emojiinfo_embed_generalinfo_id_subfield_name()?;
    let emojiinfo_embed_generalinfo_name_subfield_name =
        localizer.utilities_plugin_emojiinfo_embed_generalinfo_name_subfield_name()?;
    let emojiinfo_embed_generalinfo_guild_id_subfield_name =
        localizer.utilities_plugin_emojiinfo_embed_generalinfo_guild_id_subfield_name()?;
    let emojiinfo_embed_generalinfo_animated_subfield_name =
        localizer.utilities_plugin_emojiinfo_embed_generalinfo_animated_subfield_name()?;
    let emojiinfo_embed_generalinfo_managed_subfield_name =
        localizer.utilities_plugin_emojiinfo_embed_generalinfo_managed_subfield_name()?;

    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            emojiinfo_embed_generalinfo_field_name,
            format!(
                "{} {}\n{} {}\n{} {}\n{} {}\n{} {}",
                emojiinfo_embed_generalinfo_id_subfield_name,
                emoji.id.to_string().discord_inline_code(),
                emojiinfo_embed_generalinfo_name_subfield_name,
                emoji.name,
                emojiinfo_embed_generalinfo_guild_id_subfield_name,
                emoji.guild_id.to_string().discord_inline_code(),
                emojiinfo_embed_generalinfo_animated_subfield_name,
                emoji.animated.localize(langid_locale.clone())?,
                emojiinfo_embed_generalinfo_managed_subfield_name,
                emoji.managed.localize(langid_locale)?,
            ),
        ))
        .validate()
        .into_diagnostic()?
        .build();

    interaction_client
        .create_response(
            interaction.id,
            &interaction.token,
            &embed_response(vec![embed]),
        )
        .await
        .into_diagnostic()?;

    Ok(())
}
