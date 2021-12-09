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

//! The `emitter` Module
//!
//! This module contains an emitter used for emitting the custom events.

use crate::{
    events,
    listener
};

/// # Struct `EventEmitter`
///
/// An event emitter.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct EventEmitter {
    listeners: listener::Listeners<events::HarTexEvent>
}

impl EventEmitter {
    /// # Constructor `EventEmitter::new`
    ///
    /// Creates a new emitter with the corresponding listeners.
    ///
    /// ## Parameters
    /// - `listeners`, type `Listeners<HarTexEvent>`; the event listeners.
    #[must_use]
    pub fn new(listeners: listener::Listeners<events::HarTexEvent>) -> Self {
        Self {
            listeners
        }
    }

    /// # Constructor `EventEmitter::emit`
    ///
    /// Emits a event.
    ///
    /// ## Parameters
    /// - `event`, type `HarTexEvent`; the event to emit
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn emit(&self, event: events::HarTexEvent) {
        let count = self.listeners.len();
        let mut some_event = Some(event);

        self.send(|index| {
            if index == count {
                some_event.take().unwrap()
            }
            else {
                some_event.clone().unwrap()
            }
        });
    }

    fn send<F>(&self, mut function: F)
    where
        F: FnMut(usize) -> events::HarTexEvent {
        let mut index = 0;

        self.listeners.listeners().retain(|_, listener| {
            index += 1;

            listener.sender.unbounded_send(function(index)).is_ok()
        });
    }
}
