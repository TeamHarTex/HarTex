/* SPDX-License-Identifier: GPL-2.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # The `events` Module
//!
//! This module defines some custom events that can be sent and received within `HarTex` Discord bot.

use std::{
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{
    Stream,
    StreamExt
};
use hartex_model::payload::CommandExecuted;

/// # Struct `Events`
///
/// This is basically a wrapper around `UnboundedReceiver<HarTexEvent>`; it receives event from
/// the stream.
#[allow(clippy::module_name_repetitions)]
pub struct Events {
    receiver: UnboundedReceiver<HarTexEvent>
}

impl Events {
    /// # Constructor `Events::new`
    ///
    /// Creates a new `Events` with the given `UnboundedReceiver`
    ///
    /// ## Parameters
    /// - `receiver`, type `UnboundedReceiver`: the unbounded receiver to create this instance with
    #[must_use]
    pub fn new(receiver: UnboundedReceiver<HarTexEvent>) -> Self {
        Self {
            receiver
        }
    }
}

impl Stream for Events {
    type Item = HarTexEvent;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(context)
    }
}

/// # Enum `HarTexEvent`
///
/// An enumeration represents the various custom-defined events that is used within `HarTex`.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub enum HarTexEvent {
    /// # Enum Variant `HarTexEvent::CommandExecuted`
    ///
    /// A command is executed.
    ///
    /// ## Tuple Struct Parameters
    /// - `0`, type `Box<CommandExecuted>`: the payload of the event.
    CommandExecuted(Box<CommandExecuted>)
}
