//! # The `framework` Module
//!
//! This module contains the command framework, which glues the entire command system together.

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
    config: CommandParserConfig<'a>
}

impl<'a> CommandFramework<'a> {
    pub fn command(self, config: CommandConfig<'a>) -> Self {
        let new_conf = self.config.command(config);

        Self {
            config: new_conf
        }
    }

    pub fn build_parser(self) -> CommandParser<'a> {
        CommandParser::new(self.config)
    }
}
