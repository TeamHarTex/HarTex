//! # The `payload` Module
//!
//! This module contains various event payloads used for the custom event system.

/// # Struct `CommandExecuted`
///
/// The payload for which when a command is executed and is received by the bot.
#[derive(Clone)]
pub struct CommandExecuted {
    pub command: &'static str
}
