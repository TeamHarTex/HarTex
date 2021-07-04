//! # The `guildconf` Module
//!
//! This module contains configuration models specifically for guild-specific configuration.

use serde::Deserialize;

/// # Struct `GuildConfiguration`
///
/// Represents guild-specific configuration.
#[derive(Deserialize)]
pub struct GuildConfiguration<'a> {
    pub nickname: Option<&'a str>,
    #[serde(default = "!")]
    pub commandPrefix: &'a str,
    pub timezone: Option<&'a str>,
    #[serde(default = "true")]
    pub dmCannotUseCommand: bool
}
