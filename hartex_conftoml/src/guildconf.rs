//! # The `guildconf` Module
//!
//! This module contains configuration models specifically for guild-specific configuration.

use serde::Deserialize;

/// # Struct `GuildConfiguration`
///
/// Represents guild-specific configuration.
#[derive(Deserialize)]
pub struct GuildConfiguration {
    #[serde(default = "default_nickname")]
    pub nickname: String,
    #[serde(default = "default_cmd_prefix")]
    pub commandPrefix: String,
    #[serde(default = "default_timezone")]
    pub timezone: String,
    #[serde(default = "default_dm_cant_use_cmd")]
    pub dmCannotUseCommand: bool
}

fn default_nickname() -> String {
    String::from("HarTex")
}

fn default_cmd_prefix() -> String {
    String::from("!")
}

fn default_timezone() -> String {
    String::from("UTC")
}

fn default_dm_cant_use_cmd() -> bool {
    true
}
