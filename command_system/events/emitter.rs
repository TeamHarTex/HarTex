///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

use super::{
    events::SystemEvent,
    listener::Listeners
};

/// Represents an event emitter.
#[derive(Clone)]
crate struct CommandEventEmitter {
    listeners: Listeners<SystemEvent>
}

impl CommandEventEmitter {
    /// Creates a new emitter with a set of listeners.
    crate fn new(listeners: Listeners<SystemEvent>) -> Self {
        Self {
            listeners
        }
    }

    /// Returns the emitters that this instance of the emitter has.
    crate fn into_listeners(self) -> Listeners<SystemEvent> {
        self.listeners
    }

    /// Creates an event for the listeners
    crate fn event(&self, event: SystemEvent) {
        let listener_count = self.listeners.len();
        let mut event = Some(event);

        self.send(
            |idx| {
                if idx == listener_count {
                    event.take().unwrap()
                }
                else {
                    event.clone().unwrap()
                }
            }
        )
    }

    /// Sends the events created from the `event` method.
    fn send(&self, mut f: impl FnMut(usize) -> SystemEvent) {
        let mut idx = 0;

        self.listeners.all().retain(|_, listener| {
            idx += 1;

            listener.tx.unbounded_send(f(idx)).is_ok()
        });
    }
}
