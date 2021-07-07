//! # `hartex_conftoml` - The HarTex Configuration TOML Library.
//!
//! The `hartex_conftoml` provides an interface for serializing and deserializing TOML
//! configuration for HarTex Discord bot.

#![allow(non_snake_case)]

use serde::Deserialize;

use hartex_core::error::HarTexResult;

pub mod dashacc;
pub mod guildconf;

/// # Struct `TomlConfig`
///
/// Represents the top-level configuration, all other configuration branches from here.
#[derive(Deserialize)]
pub struct TomlConfig {
    pub DashboardAccesses: Vec<dashacc::DashboardAccess>,
    pub GuildConfiguration: guildconf::GuildConfiguration
}

pub fn from_string(input: String) -> HarTexResult<TomlConfig> {
    Ok(toml::from_str(input.as_str())?)
}
