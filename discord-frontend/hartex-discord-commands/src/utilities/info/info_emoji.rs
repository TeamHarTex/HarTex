/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::{str::FromStr, sync::LazyLock};

use hartex_discord_commands_core::context::CommandContext;
use hartex_discord_core::discord::{
    model::{
        application::interaction::application_command::CommandDataOption,
        id::{Id, marker::EmojiMarker},
    },
    util::builder::embed::{EmbedBuilder, EmbedFieldBuilder},
};
use hartex_discord_utils::{
    commands::{CommandDataOptionExt, CommandDataOptionsExt},
    interaction::{embed_response, ephemeral_error_response},
    localizable::Localizable,
    markdown::MarkdownStyle,
};
use miette::IntoDiagnostic;
use regex::Regex;

/// The regex for looking for a Discord emoji in the command input.
static EMOJI_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("<a?:[a-zA-Z0-9_]+:([0-9]{17,19})>").unwrap());

/// Executes the `info emoji` command.
#[allow(clippy::too_many_lines)]
pub async fn execute(
    context: &CommandContext<'_>,
    option: &CommandDataOption,
) -> miette::Result<()> {
    let options = option.assume_subcommand();

    let langid_locale = context
        .locale
        .as_ref()
        .and_then(|locale| locale.parse().ok());

    let emoji = options.string_value_of("emoji");

    let emojiinfo_error_only_custom_emojis = context
        .localizer
        .utilities_plugin_emojiinfo_error_only_custom_emojis()?;
    let emojiinfo_error_only_one_emoji = context
        .localizer
        .utilities_plugin_emojiinfo_error_only_one_emoji()?;
    let emojiinfo_error_unknown_emoji = context
        .localizer
        .utilities_plugin_emojiinfo_error_unknown_emoji()?;

    let Some(captures) = EMOJI_REGEX.captures(&emoji) else {
        context
            .create_response(ephemeral_error_response(emojiinfo_error_only_custom_emojis))
            .await?;

        return Ok(());
    };

    if captures.len() > 2 {
        context
            .create_response(ephemeral_error_response(emojiinfo_error_only_one_emoji))
            .await?;

        return Ok(());
    }

    let id = captures.get(1).unwrap().as_str();
    let emoji_id = Id::<EmojiMarker>::from_str(id).unwrap();

    let Some(emoji) = context.cache.emoji(emoji_id) else {
        context
            .create_response(ephemeral_error_response(emojiinfo_error_unknown_emoji))
            .await?;

        return Ok(());
    };

    let emojiinfo_embed_generalinfo_field_name = context
        .localizer
        .utilities_plugin_emojiinfo_embed_generalinfo_field_name()?;
    let emojiinfo_embed_generalinfo_id_subfield_name = context
        .localizer
        .utilities_plugin_emojiinfo_embed_generalinfo_id_subfield_name()?;
    let emojiinfo_embed_generalinfo_name_subfield_name = context
        .localizer
        .utilities_plugin_emojiinfo_embed_generalinfo_name_subfield_name()?;
    let emojiinfo_embed_generalinfo_guild_id_subfield_name = context
        .localizer
        .utilities_plugin_emojiinfo_embed_generalinfo_guild_id_subfield_name()?;
    let emojiinfo_embed_generalinfo_animated_subfield_name = context
        .localizer
        .utilities_plugin_emojiinfo_embed_generalinfo_animated_subfield_name()?;
    let emojiinfo_embed_generalinfo_managed_subfield_name = context
        .localizer
        .utilities_plugin_emojiinfo_embed_generalinfo_managed_subfield_name()?;

    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            emojiinfo_embed_generalinfo_field_name,
            format!(
                "{} {}\n{} {}\n{} {}\n{} {}\n{} {}",
                emojiinfo_embed_generalinfo_id_subfield_name,
                emoji.id().to_string().discord_inline_code(),
                emojiinfo_embed_generalinfo_name_subfield_name,
                emoji.name(),
                emojiinfo_embed_generalinfo_guild_id_subfield_name,
                emoji.guild_id().to_string().discord_inline_code(),
                emojiinfo_embed_generalinfo_animated_subfield_name,
                emoji.animated().localize(langid_locale.clone())?,
                emojiinfo_embed_generalinfo_managed_subfield_name,
                emoji.managed().localize(langid_locale)?,
            ),
        ))
        .validate()
        .into_diagnostic()?
        .build();

    context.create_response(embed_response(vec![embed])).await?;

    Ok(())
}
