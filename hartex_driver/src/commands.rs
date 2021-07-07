//! # The `commands` Module
//!
//! This module defines the command handler, which is used when a command is detected in a message.

use hartex_cmdsys::{
    command::Command,
    context::CommandContext,
    parser::ParsedCommand
};

use hartex_core::{
    discord::{
        http::Client,
        cache_inmemory::InMemoryCache
    },
    error::HarTexResult
};

use hartex_eventsys::{
    emitter::EventEmitter
};

use hartex_plugins::global::team::Team;

pub async fn handle_command(
    command: ParsedCommand<'_>,
    _: EventEmitter,
    cache: InMemoryCache,
    context: CommandContext
) -> HarTexResult<()> {
    match command {
        ParsedCommand { name: "team", args } => {
            Team::execute(context, args, cache).await?;
        },
        _ => ()
    }

    Ok(())
}
