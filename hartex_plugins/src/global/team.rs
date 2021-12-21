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

//! # The `team` Module
//!
//! This module implements the `team` command.

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
            EmbedBuilder,
            EmbedFieldBuilder
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
    STABLE
};
use hartex_dbmani::guildconf::GetGuildConfig;
use hartex_plugin_localize::global::TeamCmdLocalize;
use hartex_utils::FutureRetType;

/// # Struct `Team`
///
/// The `team` command.
pub struct Team;

impl Command for Team {
    fn name(&self) -> String {
        String::from("team")
    }

    fn description(&self) -> String {
        String::from("GlobalPlugin.TeamCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        _: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_team_command(ctx))
    }
}

/// # Asynchronous Function `exec_team_slash_cmd`
///
/// Executes the `team` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_team_command(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
        });
    };

    let (title, gb_admin_ld_dev, other_contributors) = if interaction.guild_id.is_none()
        || interaction.user.is_some()
    {
        (
            String::from("HarTex Project Team"),
            String::from("Global Administrator & Lead Developer"),
            String::from("Other Contributors")
        )
    }
    else {
        let config = GetGuildConfig::new(interaction.guild_id.unwrap()).await?;

        if !STABLE && config.NightlyFeatures.localization {
            let locale = config.GuildConfiguration.locale;
            let localize = TeamCmdLocalize::init(locale).expect("failed to load localization for team command");

            (
                localize.embed_title,
                localize.embed_globadmin_leaddev_field,
                localize.embed_contrib_field
            )
        }
        else {
            (
                String::from("HarTex Project Team"),
                String::from("Global Administrator & Lead Developer"),
                String::from("Other Contributors")
            )
        }
    };

    let embed = EmbedBuilder::new()
        .title(title)
        .color(0x0003_BEFC)
        .field(EmbedFieldBuilder::new(
            gb_admin_ld_dev,
            "HTGAzureX1212.#5959"
        ))
        .field(EmbedFieldBuilder::new(
            other_contributors,
            "<https://github.com/HarTexTeam/HarTex-rust-discord-bot/graphs/contributors>"
        ))
        .build()?;

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
