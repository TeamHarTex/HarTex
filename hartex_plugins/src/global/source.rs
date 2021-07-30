//! # The `source` Module
//!
//! This module implements the `source` command.

use hartex_cmdsys::{
    command::Command,
    context::CommandContext,
    parser::args::CommandArgs
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        model::channel::message::AllowedMentions
    },
    error::HarTexResult
};

use hartex_utils::FutureRetType;

/// # Struct `Source`
///
/// The `source` command.
pub struct Source;

impl Command for Source {
    fn name() -> String {
        String::from("source")
    }

    fn execute(ctx: CommandContext, _: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
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
    ctx.http
        .create_message(ctx.message.channel_id)
        .allowed_mentions(AllowedMentions::default())
        .content("The source code for the bot can be found at: <https://github.com/HT-Studios/HarTex-rust-discord-bot>.")?
        .reply(ctx.message.id)
        .await?;

    Ok(())
}