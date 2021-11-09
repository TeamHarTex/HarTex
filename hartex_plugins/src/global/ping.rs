//! # The `ping` Module
//!
//! This module implements the `ping` command.

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
    logging::tracing
};
use hartex_utils::{
    shard_id,
    FutureRetType
};

/// # Struct `Ping`
///
/// The `ping` command.
pub struct Ping;

impl Command for Ping {
    fn name(&self) -> String {
        String::from("ping")
    }

    fn description(&self) -> String {
        String::from("GlobalPlugin.PingCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        _: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_ping_command(ctx))
    }
}

/// # Asynchronous Function `exec_ping_slash_cmd`
///
/// Executes the `ping` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_ping_command(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
        });
    };

    let content = String::from("Hello! Did you need anything? :eyes:");

    tracing::trace!("responding to interaction (initial response)");

    if let Err(error) = ctx
        .http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: Some(content.clone()),
                embeds: vec![],
                flags: None,
                tts: None
            })
        )
        .exec()
        .await
    {
        tracing::error!("failed to create initial interaction response: {error}");

        return Err(HarTexError::from(error));
    }

    tracing::trace!("obtaining latency information");

    let shards = ctx.cluster.info();
    let shard_id = shard_id(interaction.guild_id.unwrap().0.get(), shards.len() as _);
    let shard_info = shards.get(&shard_id).unwrap();
    let latency = shard_info.latency().average().unwrap();
    let new_content = format!("{content} - `{latency}ms`", latency = latency.as_millis());

    tracing::trace!("updating initial interaction response to add latency information");

    if let Err(error) = {
        match ctx
            .http
            .update_interaction_original(&interaction.token)?
            .content(Some(&new_content))
        {
            Ok(update) => update,
            Err(error) => {
                return Err(HarTexError::Custom {
                    message: format!("failed to update original response: {error}")
                });
            }
        }
        .exec()
        .await
    } {
        tracing::error!("failed to update initial interaction response: {error}");

        return Err(HarTexError::from(error));
    }

    Ok(())
}
