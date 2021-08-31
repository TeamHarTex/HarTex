//! # The `source` Module
//!
//! This module implements the `source` command.

use hartex_cmdsys::{
    command::{
        Command,
        SlashCommand
    },
    context::CommandContext,
    parser::args::CommandArgs
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        model::{
            application::{
                callback::{
                    CallbackData,
                    InteractionResponse
                },
                interaction::Interaction
            },
            channel::message::AllowedMentions
        }
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_utils::FutureRetType;

/// # Struct `Source`
///
/// The `source` command.
pub struct Source;

impl Command for Source {
    fn name(&self) -> String {
        String::from("source")
    }

    fn execute_command(&self, ctx: CommandContext, _: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
        Box::pin(exec_source_cmd(ctx))
    }
}

/// # Asynchronous Function `exec_source_cmd`
///
/// Executes the `source` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn exec_source_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let message = ctx.message.clone().unwrap();

    ctx.http
        .create_message(message.channel_id)
        .allowed_mentions(AllowedMentions::default())
        .content("The source code for the bot can be found at: <https://github.com/HT-Studios/HarTex-rust-discord-bot>.")?
        .reply(message.id)
        .exec()
        .await?;

    Ok(())
}

impl SlashCommand for Source {
    fn description(&self) -> String {
        String::from("GlobalPlugin.SourceCommand")
    }

    fn execute_slash_command<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(exec_source_slash_cmd(ctx))
    }
}

async fn exec_source_slash_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = match ctx.interaction.as_ref().unwrap() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(
                        String::from(
                            "The source code for the bot can be found at: <https://github.com/HT-Studios/HarTex-rust-discord-bot>."
                        )
                    ),
                    embeds: vec![],
                    flags: None,
                    tts: None
                }
            )
        )
        .exec()
        .await?;

    Ok(())
}
