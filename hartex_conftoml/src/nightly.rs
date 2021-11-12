//! # The `nightly` Module
//!
//! This module contains configuration for opt-in nightly unstable features that are in the testing
//! phase.
//!
//! This API is very unstable and may change rapidly as features are added into the bot.

use serde::Deserialize;

/// # Struct `NightlyFeatures`
///
/// The opt-in nightly features that the bot provides.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct NightlyFeatures {
    // Experimental Support for the Discord Threads API
    #[serde(default = "default_feature_enabled")]
    pub threads: bool,
    // Experimental Support for Localization Facilities, i.e. timezones, languages
    #[serde(default = "default_feature_enabled")]
    pub localization: bool
}

#[must_use]
pub fn default_feature_enabled() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use super::NightlyFeatures;

    #[test]
    fn test_nightly_de() {
        serde_test::assert_de_tokens(
            &NightlyFeatures {
                threads: false,
                localization: true
            },
            &[
                Token::Struct {
                    name: "NightlyFeatures",
                    len: 2
                },
                Token::Str("threads"),
                Token::Bool(false),
                Token::Str("localization"),
                Token::Bool(true),
                Token::StructEnd
            ]
        );
    }
}
