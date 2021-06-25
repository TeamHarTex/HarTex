//! # The `framework` Module
//!
//! This module contains the command framework, which glues the entire command system together.

use crate::parser::{
    config::CommandParserConfig
};

/// # Struct `CommandFramework`
///
/// The command framework.
#[derive(Clone)]
pub struct CommandFramework {
    config: CommandParserConfig
}

impl CommandFramework {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            config: Default::default()
        }
    }
}
