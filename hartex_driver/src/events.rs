//! # The `events` Module
//!
//! The `events` module provides utility functions for handling events as they come into the event
//! loop.

use hartex_core::{
    discord::gateway::Event,
    error::HarTexResult,
    events::EventType
};

async fn handle_event(_shard_id: u64, (_event_type, _twilight): (EventType, Option<Event>)) -> HarTexResult<()> {
    Ok(())
}
