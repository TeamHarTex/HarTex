//! # The `events` Module
//!
//! The `events` module provides utility functions for handling events as they come into the event
//! loop.

use hartex_core::{
    discord::gateway::Event,
    error::HarTexResult,
    events::EventType
};

use crate::handler::EventHandler;

pub async fn handle_event(_shard_id: u64, (event_type, twilight): (EventType, Option<Event>)) -> HarTexResult<()> {
    match event_type {
        EventType::Twilight if twilight.is_some() => {
            match twilight.unwrap() {
                Event::Ready(payload) => {
                    EventHandler::ready(payload).await?
                },
                Event::ShardIdentifying(payload) => {
                    EventHandler::shard_identifying(payload).await?
                }
                _ => ()
            }
        },
        _ => todo!()
    }

    Ok(())
}
