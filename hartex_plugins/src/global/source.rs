//! # The `source` Module
//!
//! This module implements the `source` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
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
    },
    logging::tracing
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

    fn description(&self) -> String {
        String::from("GlobalPlugin.SourceCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
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
    let interaction = match ctx.interaction.clone() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    tracing::trace!("responding to interaction");

    if let Err(error) = ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(
                        String::from(
                            "The source code for the bot can be found at: <https://github.com/HarTexBot/HarTex-rust-discord-bot>."
                        )
                    ),
                    embeds: vec![],
                    flags: None,
                    tts: None
                }
            )
        )
        .exec()
        .await {
        tracing::error!("failed to respond to interaction: {error}");

        return Err(HarTexError::from(error));
    }

    Ok(())
}
