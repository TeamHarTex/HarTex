//! # The `commands` Module
//!
//! This module defines the command handler, which is used when a command is detected in a message.

use hartex_cmdsys::parser::ParsedCommand;

use hartex_core::{
    discord::{
        http::Client,
        cache_inmemory::InMemoryCache
    },
    error::HarTexResult
};

use hartex_eventsys::emitter::EventEmitter;

pub async fn handle_command(
    command: ParsedCommand<'_>,
    emitter: EventEmitter,
    http: Client,
    cache: InMemoryCache
) -> HarTexResult<()> {
    Ok(())
}
