/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex; if not, If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `events` Module
//!
//! The `events` module provides utility functions for handling events as they come into the event
//! loop.

use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        gateway::{
            CloneableCluster,
            Event
        },
        http::CloneableClient
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

/// # Asynchronous Function `handle_event`
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
/// - `cluster`, type `Cluster`: the gateway cluster to use for some specific events that need it
///
/// ## Errors
///
/// Returns handler-related errors.
#[allow(clippy::missing_panics_doc)] // this function never panics
#[allow(clippy::semicolon_if_nothing_returned)]
pub async fn handle_event(
    (event_type, twilight, custom): (EventType, Option<Event>, Option<HarTexEvent>),
    http: CloneableClient,
    emitter: EventEmitter,
    cache: CloneableInMemoryCache,
    cluster: CloneableCluster
) -> HarTexResult<()> {
    match event_type {
        EventType::Twilight if twilight.is_some() => match twilight.unwrap() {
            Event::GuildCreate(payload) => EventHandler::guild_create(payload, http).await?,
            Event::InteractionCreate(payload) => {
                EventHandler::interaction_create(payload, http, cluster, cache, emitter).await?
            }
            Event::MessageCreate(payload) => {
                EventHandler::message_create(payload, emitter, cache, http, cluster).await?
            }
            Event::Ready(payload) => EventHandler::ready(payload, cluster, http).await?,
            Event::ShardIdentifying(payload) => EventHandler::shard_identifying(payload).await?,
            _ => ()
        },
        EventType::Custom if custom.is_some() => match custom.unwrap() {
            HarTexEvent::CommandExecuted(payload) => EventHandler::command_executed(payload).await?
        },
        _ => {
            return Err(HarTexError::Custom {
                message: String::from("event type mismatch")
            });
        }
    }

    Ok(())
}
