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

use crate::parser::{
    config::{
        CommandConfig,
        CommandParserConfig
    },
    CommandParser
};

/// # Struct `CommandFramework`
///
/// The command framework.
#[derive(Clone, Default)]
pub struct CommandFramework<'a> {
    config: CommandParserConfig<'a>,
    listeners: Listeners<HarTexEvent>
}

impl<'a> CommandFramework<'a> {
    /// # Instance Method `CommandFramework::command`
    ///
    /// Adds a command to the parser configuration.
    ///
    /// ## Parameters
    /// - `config`, type `CommandConfig`: the configuration of the command to add.
    pub fn command(self, config: CommandConfig<'a>) -> Self {
        let new_conf = self.config.command(config);

        Self {
            config: new_conf,
            listeners: Listeners::default()
        }
    }

    /// # Instance Method `CommandFramework::build_parser`
    ///
    /// Builds a command parser from the configuration and consumes the framework.
    pub fn build_parser(self) -> CommandParser<'a> {
        CommandParser::new(self.config)
    }

    /// # Instance Method `CommandFramework::events`
    ///
    /// Returns a stream of events and consumes the framework.
    pub fn events(self) -> Events {
        let receiver = self.listeners.add();

        Events::new(receiver)
    }

    /// # Instance Method `CommandFramework::listeners`
    ///
    /// Returns the listeners of the current framework and consumes it.
    pub fn listeners(self) -> Listeners<HarTexEvent> {
        self.listeners
    }
}
