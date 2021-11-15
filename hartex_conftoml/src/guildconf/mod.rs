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
#[derive(Debug, Deserialize, PartialEq)]
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

fn deserialize_timezone<'deserialize, D>(
    deserializer: D
) -> Result<tz::Timezone, D::Error>
where
    D: de::Deserializer<'deserialize> {
    deserializer.deserialize_str(tz::GuildConfigTimezoneDeserializerRefStrVisitor)
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use super::{
        tz,
        GuildConfiguration
    };

    #[test]
    fn test_guildconf_de() {
        serde_test::assert_de_tokens(
            &GuildConfiguration {
                nickname: String::from("HarTex"),
                timezone: tz::Timezone::UTC,
                dmCannotUseCommand: true
            },
            &[
                Token::Struct {
                    name: "GuildConfiguration",
                    len: 3
                },
                Token::Str("nickname"),
                Token::Str("HarTex"),
                Token::Str("timezone"),
                Token::Str("UTC"),
                Token::Str("dmCannotUseCommand"),
                Token::Bool(true),
                Token::StructEnd
            ]
        );
    }
}
