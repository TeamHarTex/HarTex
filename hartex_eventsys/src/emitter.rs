//! The `emitter` Module
//!
//! This module contains an emitter used for emitting the custom events.

use crate::{
    events,
    listener
};

#[derive(Clone)]
pub struct EventEmitter<'a> {
    listeners: listener::Listeners<events::HarTexEvent<'a>>
}

impl<'a> EventEmitter<'a> {
    pub fn new(listeners: listener::Listeners<events::HarTexEvent<'a>>) -> Self {
        Self {
            listeners
        }
    }

    pub fn emit(&self, event: events::HarTexEvent<'a>) {
        let count = self.listeners.len();
        let mut someevent = Some(event);

        self.send(|index| {
            if index == count {
                someevent.take().unwrap()
            }
            else {
                someevent.clone().unwrap()
            }
        })
    }

    fn send<F>(&self, mut function: F)
    where
        F: FnMut(usize) -> events::HarTexEvent<'a> {
        let mut index = 0;

        self.listeners.listeners().retain(|_, listener| {
            index += 1;

            listener.sender.unbounded_send(function(index)).is_ok()
        });
    }
}
