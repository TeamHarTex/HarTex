/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `about` Module
//!
//! This module implements the `about` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};
use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        embed_builder::{
            EmbedAuthorBuilder,
            EmbedBuilder,
            EmbedFieldBuilder,
            ImageSource
        },
        model::application::{
            callback::{
                CallbackData,
                InteractionResponse
            },
            interaction::Interaction
        }
    },
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing,
    HARTEX_BUILD,
    STABLE
};
use hartex_dbmani::{
    guildconf::GetGuildConfig,
    whitelist::GetWhitelistedGuilds
};
use hartex_plugin_localize::global::AboutCmdLocalize;
use hartex_utils::FutureRetType;

/// # Struct `About`
///
/// The `about` command.
pub struct About;

impl Command for About {
    fn name(&self) -> String {
        String::from("about")
    }

    fn description(&self) -> String {
        String::from("GlobalPlugin.AboutCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        _: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_about_command(ctx))
    }
}

/// # Asynchronous Function `execute_about_command`
///
/// Executes the `about` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_about_command(ctx: CommandContext) -> HarTexResult<()> {
    tracing::trace!("attempting to obtain whitelisted guilds");

    let whitelists = match GetWhitelistedGuilds::default().await {
        Ok(list) => list.len(),
        Err(error) => {
            tracing::trace!("failed to obtain whitelisted guilds {error:?}");

            return Err(error);
        }
    };
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
        });
    };

    let builder = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new(String::from("HarTex"))
            .icon_url(ImageSource::url("https://cdn.discordapp.com/attachments/795539269925601341/862616114239897610/275a4a2ecfb5380a45c393c81838c14b.png")?)
        );

    let embed = if interaction.guild_id.is_none() || interaction.user.is_some() {
        builder
            .description("HarTex is a Discord bot built and optimized for efficient Discord moderation and administration, maintained by the HarTex Development Team members.")
            .color(0x0003_BEFC)
            .field(EmbedFieldBuilder::new("Bot Version", HARTEX_BUILD))
            .field(EmbedFieldBuilder::new("Whitelisted Guilds", whitelists.to_string()).inline().build())
            .build()?
    }
    else {
        let config = GetGuildConfig::new(interaction.guild_id.unwrap()).await?;

        if !STABLE && config.NightlyFeatures.localization {
            let locale = config.GuildConfiguration.locale;
            let localize = AboutCmdLocalize::init(locale)
                .expect("failed to load localization for about command");

            builder
                .description(localize.embed_desc)
                .color(0x0003_BEFC)
                .field(EmbedFieldBuilder::new(
                    localize.embed_botver_field,
                    HARTEX_BUILD
                ))
                .field(
                    EmbedFieldBuilder::new(
                        localize.embed_whiteguilds_field,
                        whitelists.to_string()
                    )
                    .inline()
                    .build()
                )
                .build()?
        }
        else {
            builder
                .description("HarTex is a Discord bot built and optimized for efficient Discord moderation and administration, maintained by the HarTex Development Team members.")
                .color(0x0003_BEFC)
                .field(EmbedFieldBuilder::new("Bot Version", HARTEX_BUILD))
                .field(EmbedFieldBuilder::new("Whitelisted Guilds", whitelists.to_string()).inline().build())
                .build()?
        }
    };

    tracing::trace!("responding to interaction");

    if let Err(error) = ctx
        .http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: None,
                embeds: vec![embed],
                flags: None,
                tts: None
            })
        )
        .exec()
        .await
    {
        tracing::error!("failed to respond to interaction: {error}");

        return Err(HarTexError::from(error));
    }

    Ok(())
}
