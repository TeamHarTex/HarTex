//! # `hartex_conftoml` - The HarTex Configuration TOML Library.
//!
//! The `hartex_conftoml` provides an interface for serializing and deserializing TOML
//! configuration for HarTex Discord bot.

#![allow(non_snake_case)]

use serde::Deserialize;

pub mod dashacc;
pub mod guildconf;

/// # Struct `TomlConfig`
///
/// Represents the top-level configuration, all other configuration branches from here.
#[derive(Deserialize)]
pub struct TomlConfig<'a> {
    pub dashboardAccesses: Vec<dashacc::DashboardAccess>,
    pub guildConfiguration: guildconf::GuildConfiguration<'a>
}
