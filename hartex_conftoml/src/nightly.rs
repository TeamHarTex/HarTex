//! # The `nightly` Module
//!
//! This module contains configuration for opt-in nightly unstable features that are in the testing
//! phase.
//!
//! This API is very unstable and may change rapidly as features are added into the bot.

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct NightlyFeatures {
    #[serde(default = "default_feature_enabled")]
    pub interactions: bool
}

pub fn default_feature_enabled() -> bool {
    false
}
