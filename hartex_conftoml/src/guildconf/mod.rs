//! # The `guildconf` Module
//!
//! This module contains configuration models specifically for guild-specific configuration.

use serde::{
    de,
    Deserialize
};

pub mod tz;

/// # Struct `GuildConfiguration`
///
/// Represents guild-specific configuration.
#[derive(Deserialize)]
pub struct GuildConfiguration {
    #[serde(default = "default_nickname")]
    pub nickname: String,
    #[serde(
        default = "default_timezone",
        deserialize_with = "deserialize_timezone"
    )]
    pub timezone: tz::Timezone,
    #[serde(default = "default_dm_cant_use_cmd")]
    pub dmCannotUseCommand: bool
}

fn default_nickname() -> String {
    String::from("HarTex")
}

fn default_timezone() -> tz::Timezone {
    tz::Timezone::UTC
}

fn default_dm_cant_use_cmd() -> bool {
    true
}

fn deserialize_timezone<'deserialize, Deserializer>(
    deserializer: Deserializer
) -> Result<tz::Timezone, Deserializer::Error>
where
    Deserializer: de::Deserializer<'deserialize> {
    deserializer.deserialize_str(tz::TimezoneDeserializeStringVisitor)
}
