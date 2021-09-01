//! # The `source` Module
//!
//! This module implements the `source` command.

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

use hartex_utils::FutureRetType;

/// # Struct `Source`
///
/// The `source` command.
pub struct Source;

impl SlashCommand for Source {
    fn name(&self) -> String {
        String::from("source")
    }

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
