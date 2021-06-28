//! # The `payload` Module
//!
//! This module contains various event payloads used for the custom event system.

#[path = "../../hartex_cmdsys/src/context.rs"]
pub mod context;

use hartex_core::discord::model::guild::Guild;

/// # Struct `CommandExecuted`
///
/// The payload for which when the command is executed.
pub struct CommandExecuted<'a> {
    pub command: &'a str,
    pub guild: Guild,
    pub context: context::CommandContext
}
