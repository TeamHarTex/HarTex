//! # The `events` Module
//!
//! The `events` module provides utility functions for handling events as they come into the event
//! loop.

use hartex_core::{
    discord::{
        gateway::Event,
        http::Client
    },
    error::{
        HarTexError,
        HarTexResult
    },
    events::EventType
};

use hartex_eventsys::{
    emitter::EventEmitter,
    events::HarTexEvent
};

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
/// - `http`, type `Client`: the Twilight HTTP Client to use for some specific events that need it
#[allow(clippy::needless_lifetimes)]
pub async fn handle_event(
    (event_type, twilight, custom): (EventType, Option<Event>, Option<HarTexEvent>),
    http: Client,
    emitter: EventEmitter) -> HarTexResult<()> {
    match event_type {
        EventType::Twilight if twilight.is_some() => {
            match twilight.unwrap() {
                Event::GuildCreate(payload) => {
                    EventHandler::guild_create(payload, http).await?
                }
                Event::MessageCreate(payload) => {
                    EventHandler::message_create(payload, emitter).await?
                }
                Event::Ready(payload) => {
                    EventHandler::ready(payload).await?
                }
                Event::ShardIdentifying(payload) => {
                    EventHandler::shard_identifying(payload).await?
                }
                _ => ()
            }
        },
        EventType::Custom if custom.is_some() => {
            match custom.unwrap() {
                HarTexEvent::CommandExecuted(payload) => {
                    EventHandler::command_executed(payload).await?
                }
            }
        }
        _ => return Err(HarTexError::Custom {
            message: String::from("event type mismatch")
        })
    }

    Ok(())
}
