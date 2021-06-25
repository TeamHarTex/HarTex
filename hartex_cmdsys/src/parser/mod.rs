//! # The `parser` Module
//!
//! This module contains an implementation of a command parser.

pub mod args;
pub mod config;

/// # Struct `CommandParser`
///
/// The command parser.
pub struct CommandParser {
    config: config::CommandParserConfig
}

impl CommandParser {
    pub fn new(config: config::CommandParserConfig) -> Self {
        Self {
            config
        }
    }
}
