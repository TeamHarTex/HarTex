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
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! A command framework, which glues the entire command system together.

use hartex_eventsys::{
    events::{Events, HarTexEvent},
    listener::Listeners,
};

/// The command framework.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct CommandFramework {
    listeners: Listeners<HarTexEvent>,
}

impl CommandFramework {
    /// A stream of events and consumes the framework.
    #[must_use]
    pub fn events(self) -> Events {
        let receiver = self.listeners.add();

        Events::new(receiver)
    }

    /// The listeners of the current framework and consumes it.
    #[must_use]
    pub fn listeners(self) -> Listeners<HarTexEvent> {
        self.listeners
    }
}
