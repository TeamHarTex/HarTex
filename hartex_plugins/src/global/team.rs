//! # The `team` Module
//!
//! This module implements the `team` command.

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
        embed_builder::{
            EmbedBuilder,
            EmbedFieldBuilder
        },
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

/// # Struct `Team`
///
/// The `team` command.
pub struct Team;

impl Command for Team {
    fn name(&self) -> String {
        String::from("team")
    }

    fn execute_command(&self, ctx: CommandContext, _: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
        Box::pin(exec_team_cmd(ctx))
    }
}

/// # Asynchronous Function `exec_team_cmd`
///
/// Executes the `team` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn exec_team_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let message = ctx.message.clone().unwrap();
    let embed = EmbedBuilder::new()
        .title("HarTex Project Team")
        .color(0x03BEFC)
        .field(EmbedFieldBuilder::new("Global Administrator & Lead Developer", "HTGAzureX1212.#5959"))
        .build()?;

    ctx.http
        .create_message(message.channel_id)
        .allowed_mentions(AllowedMentions::default())
        .embeds(&[embed])?
        .reply(message.id)
        .exec()
        .await?;

    Ok(())
}

impl SlashCommand for Team {
    fn description(&self) -> String {
        String::from("GlobalPlugin.TeamCommand")
    }

    fn execute_slash_command<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(exec_team_slash_cmd(ctx))
    }
}

/// # Asynchronous Function `exec_team_slash_cmd`
///
/// Executes the `team` command (the slash command variant).
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn exec_team_slash_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = match ctx.interaction.as_ref().unwrap() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    let embed = EmbedBuilder::new()
        .title("HarTex Project Team")
        .color(0x03BEFC)
        .field(EmbedFieldBuilder::new("Global Administrator & Lead Developer", "HTGAzureX1212.#5959"))
        .build()?;


    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: None,
                    embeds: vec![embed],
                    flags: None,
                    tts: None
                }
            )
        )
        .exec()
        .await?;

    Ok(())
}
