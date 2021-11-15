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
