//! # The `config` Module
//!
//! This module contains configuration utilities for the command parser.

/// # Struct `CommandParserConfig`
///
/// The command parser configuration.
#[derive(Clone, Debug, Default)]
pub struct CommandParserConfig {
    commands: Vec<String>
}
