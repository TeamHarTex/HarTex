//! # The `events` Module
//!
//! The `events` module provides utility functions for handling events as they come into the event
//! loop.

use hartex_core::{
    discord::gateway::Event,
    error::HarTexResult,
    events::EventType
};

use hartex_logging::Logger;

pub async fn handle_event(shard_id: u64, (event_type, twilight): (EventType, Option<Event>)) -> HarTexResult<()> {
    Logger::verbose(format!("shard {} received an event; handling event", shard_id), Some(module_path!()));

    match event_type {
        EventType::Twilight if twilight.is_some() => {

        },
        _ => todo!()
    }

    Ok(())
}
