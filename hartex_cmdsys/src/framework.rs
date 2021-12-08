/*
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

//! # The `framework` Module
//!
//! This module contains the command framework, which glues the entire command system together.

use hartex_eventsys::{
    events::{
        Events,
        HarTexEvent
    },
    listener::Listeners
};

/// # Struct `CommandFramework`
///
/// The command framework.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct CommandFramework {
    listeners: Listeners<HarTexEvent>
}

impl CommandFramework {
    /// # Instance Method `CommandFramework::events`
    ///
    /// Returns a stream of events and consumes the framework.
    #[must_use]
    pub fn events(self) -> Events {
        let receiver = self.listeners.add();

        Events::new(receiver)
    }

    /// # Instance Method `CommandFramework::listeners`
    ///
    /// Returns the listeners of the current framework and consumes it.
    #[must_use]
    pub fn listeners(self) -> Listeners<HarTexEvent> {
        self.listeners
    }
}
