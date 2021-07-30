//! # The `team` Module
//!
//! This module implements the `team` command.

use hartex_cmdsys::{
    command::Command,
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
        model::channel::message::AllowedMentions
    },
    error::HarTexResult
};

use hartex_utils::FutureRetType;

/// # Struct `Team`
///
/// The `team` command.
pub struct Team;

impl Command for Team {
    fn name() -> String {
        String::from("team")
    }

    fn execute(ctx: CommandContext, _: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
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
        .embeds(vec![embed])?
        .reply(message.id)
        .await?;

    Ok(())
}
