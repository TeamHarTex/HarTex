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
