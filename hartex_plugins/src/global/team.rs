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
        cache_inmemory::InMemoryCache,
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
    logging::tracing
};
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
        _: InMemoryCache
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
    let interaction = match ctx.interaction.clone() {
        Interaction::ApplicationCommand(command) => command,
        _ => {
            tracing::error!("invalid interaction type: expected ApplicationCommand");

            return Err(HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            });
        }
    };

    let embed = EmbedBuilder::new()
        .title("HarTex Project Team")
        .color(0x03BEFC)
        .field(EmbedFieldBuilder::new(
            "Global Administrator & Lead Developer",
            "HTGAzureX1212.#5959"
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
