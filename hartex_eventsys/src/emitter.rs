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
