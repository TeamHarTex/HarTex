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

//! # The `source` Module
//!
//! This module implements the `source` command.

use hartex_base::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
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
    is_stable,
    logging::tracing
};
use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};
use hartex_conftoml::guildconf::locale::Locale;
use hartex_dbmani::guildconf::GetGuildConfig;
use hartex_plugin_localize::global::SourceCmdLocalize;
use hartex_utils::FutureRetType;

/// # Struct `Source`
///
/// The `source` command.
pub struct Source;

impl Command for Source {
    fn name(&self) -> String {
        String::from("source")
    }

    fn description(&self) -> String {
        String::from("GlobalPlugin.SourceCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        _: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_source_command(ctx))
    }
}

/// # Asynchronous Function `execute_source_command`
///
/// Executes the `source` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_source_command(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
        });
    };

    let localize = if interaction.guild_id.is_none() || interaction.user.is_some() {
        SourceCmdLocalize::init(Locale::EnGb)
            .expect("failed to load localization for source command")
    }
    else {
        let config = GetGuildConfig::new(interaction.guild_id.unwrap()).await?;

        if !is_stable() && config.NightlyFeatures.localization {
            SourceCmdLocalize::init(config.GuildConfiguration.locale)
                .expect("failed to load localization for source command")
        }
        else {
            SourceCmdLocalize::init(Locale::EnGb)
                .expect("failed to load localization for source command")
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
                content: Some(format!(
                    "{message} <https://github.com/HarTexTeam/HarTex-rust-discord-bot>.",
                    message = localize.prerepo_uri_msg
                )),
                embeds: vec![],
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
