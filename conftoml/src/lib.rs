/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # `hartex_conftoml` - The `HarTex` Configuration TOML Library.
//!
//! The `hartex_conftoml` provides an interface for serializing and deserializing TOML
//! configuration for `HarTex` Discord bot.

#![allow(non_snake_case)]
#![deny(clippy::pedantic, warnings)]
#![forbid(unsafe_code)]

use hartex_base::{
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing
};
use serde::Deserialize;

pub mod dashacc;
pub mod guildconf;
pub mod nightly;
pub mod permlvl;

/// # Struct `TomlConfig`
///
/// Represents the top-level configuration, all other configuration branches from here.
#[derive(Deserialize)]
pub struct TomlConfig {
    pub DashboardAccess: Vec<dashacc::DashboardAccess>,
    pub GuildConfiguration: guildconf::GuildConfiguration,
    #[serde(default)]
    pub NightlyFeatures: nightly::NightlyFeatures,
    #[serde(default)]
    pub PermissionLevels: permlvl::PermissionLevels
}

/// # Function `from_str`
///
/// Deserializes the TOML string.
///
/// ## Errors
///
/// Returns deserialization-related errors.
pub fn from_str(input: &str) -> HarTexResult<TomlConfig> {
    Ok(match toml::from_str(input) {
        Ok(config) => config,
        Err(error) => {
            tracing::error!("failed to deserialize config: {error}");

            return Err(HarTexError::from(error));
        }
    })
}
