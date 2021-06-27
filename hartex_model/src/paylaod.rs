//! # The `payload` Module
//!
//! This module contains various event payloads used for the custom event system.

use hartex_cmdsys::context::CommandContext;

use hartex_core::discord::model::guild::Guild;

/// # Struct `CommandExecuted`
///
/// The payload for which when the command is executed.
pub struct CommandExecuted<'a> {
    pub command: &'a str,
    pub guild: Guild,
    pub context: CommandContext<'a>
}
