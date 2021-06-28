//! # The `events` Module
//!
//! The `events` module provides utility functions for handling events as they come into the event
//! loop.

use hartex_core::{
    discord::gateway::Event,
    error::{
        HarTexError,
        HarTexResult
    },
    events::EventType
};

use hartex_eventsys::events::HarTexEvent;

use crate::handler::EventHandler;

/// # Function `handle_event`
///
/// Handles the incoming event asynchronously.
///
/// ## Parameters
/// - `shard_id`, type `u64`: the shard id of the shard that received the event
/// - `event_type`, type `EventType`: the event type of the event, whether it is a custom event or
///                                   a twilight event
/// - `twilight`, type `Option<Event>`: the twilight event; should only be set to `Some(...)` when
///                                     the `event_type` parameter is set to `EventType::twilight`
/// - `custom`, type `Option<HarTexEvent>`: the custom event; should only be set to `Some(...)`
///                                         when the `event_type` parameter is set to
///                                         `EventType::Custom`
#[allow(clippy::needless_lifetimes)]
pub async fn handle_event<'a>((event_type, twilight, custom): (EventType, Option<Event>, Option<HarTexEvent<'a>>)) -> HarTexResult<'a, ()> {
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
        EventType::Custom if custom.is_some() => {
            todo!()
        }
        _ => return Err(HarTexError::Custom {
            message: "event type mismatch"
        })
    }

    Ok(())
}
