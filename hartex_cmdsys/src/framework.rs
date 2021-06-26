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
pub struct CommandFramework {
    config: CommandParserConfig
}

impl CommandFramework {
    pub fn command(self, config: CommandConfig) -> Self {
        let new_conf = self.config.command(config);

        Self {
            config: new_conf
        }
    }

    pub fn build_parser(self) -> CommandParser {
        CommandParser::new(self.config)
    }
}
