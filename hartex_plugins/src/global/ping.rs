//! # The `ping` Module
//!
//! This module implements the `ping` command.

use hartex_cmdsys::{
    command::SlashCommand,
    context::CommandContext
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
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
    }
};

use hartex_utils::{
    FutureRetType,
    shard_id
};

/// # Struct `Ping`
///
/// The `ping` command.
pub struct Ping;

impl SlashCommand for Ping {
    fn name(&self) -> String {
        String::from("ping")
    }

    fn description(&self) -> String {
        String::from("GlobalPlugin.PingCommand")
    }

    fn execute_slash_command<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(exec_ping_slash_cmd(ctx))
    }
}

/// # Asynchronous Function `exec_ping_slash_cmd`
///
/// Executes the `ping` command (the slash command variant).
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn exec_ping_slash_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = match ctx.interaction.as_ref().unwrap() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    let content = String::from("Hello! Did you need anything? :eyes:");

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(content.clone()),
                    embeds: vec![],
                    flags: None,
                    tts: None
                }
            )
        )
        .exec()
        .await?;

    let shards = ctx.cluster.info();
    let shard_id = shard_id(interaction.guild_id.unwrap().0, shards.len() as _);
    let shard_info = shards
        .get(&shard_id)
        .unwrap();
    let latency = shard_info.latency().average()
        .unwrap();
    let new_content = format!("{} - `{}ms`", content, latency.as_millis());

    match ctx.http
        .update_interaction_original(&interaction.token)?
        .content(Some(&new_content)) {
        Ok(update) => update,
        Err(error) => {
            return Err(HarTexError::Custom {
                message: format!("failed to update original response: {}", error)
            });
        }
    }
        .exec()
        .await?;

    Ok(())
}
