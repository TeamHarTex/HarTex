//! # The `guildconf` Module
//!
//! This module contains configuration models specifically for guild-specific configuration.

use serde::Deserialize;

/// # Struct `GuildConfiguration`
///
/// Represents guild-specific configuration.
#[derive(Deserialize)]
pub struct GuildConfiguration {
    pub nickname: Option<String>,
    #[serde(default = "!")]
    pub commandPrefix: String,
    pub timezone: Option<String>,
    #[serde(default = "true")]
    pub dmCannotUseCommand: bool
}
