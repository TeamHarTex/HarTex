//! # The `payload` Module
//!
//! This module contains various event payloads used for the custom event system.

use hartex_core::discord::model::id::GuildId;

/// # Struct `CommandExecuted`
///
/// The payload for which when the command is executed.
#[derive(Clone)]
pub struct CommandExecuted {
    pub command: String,
    pub guild_id: GuildId
}
