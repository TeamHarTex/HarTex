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
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The HarTex Base Library
//!
//! The HarTex Base Library is the foundation of the entire HarTex bot. It is the glue between
//! various libraries that the bot utilises often, and the functionalities of the bot itself.
//!
//! The base library is *minimal*. It only re-exports some commonly-used libraries used throughout
//! the codebase (often gated behind feature flags to avoid redundant exports); while also
//! providing various primitive types and sometimes extensions to The Rust Standard Library.
//!
//! ## Feature Flags
//!
//! ### `twilight-bundled`
//!
//! This feature flag enables the bundling of the `twilight-*` ecosystem libraries within the base
//! library. This feature is off by default.
//!
//! To enable this feature, add the following line in your `Cargo.toml` file:
//! ```toml
//! hartex_base = { git = "https://github.com/HarTexTeam/HarTex-rust-discord-bot.git", features = [ "twilight-bundled" ] }
//! ```
//!
//! ### `tracing-bundled`
//!
//! This feature flag enables the bundling of the `tracing` and `tracing_subscriber` libraries
//! within the base library. This feature is off by default.
//!
//! To enable this feature, add the following line in your `Cargo.toml` file:
//! ```toml
//! hartex_base = { git = "https://github.com/HarTexTeam/HarTex-rust-discord-bot.git", features = [ "tracing-bundled" ] }
//! ```

#![deny(clippy::pedantic, warnings)]
#![forbid(unsafe_code)]

pub use ctrlc;

#[cfg(feature = "twilight-bundled")]
pub mod discord;
pub mod error;
pub mod events;
#[cfg(feature = "tracing-bundled")]
pub mod logging;
pub mod time;

/// The current version of the bot; corresponds to the `CFG_VERSION_STR` environment variable at
/// compile time.
#[must_use]
pub fn hartex_version() -> &'static str {
    env!("CFG_VERSION_STR")
}

/// Whether this particular build of the bot is stable (i.e. whether nightly features are enabled).
/// Corresponds to the `CFG_IS_STABLE` environment variable at compile time.
#[must_use]
pub fn is_stable() -> bool {
    matches!(env!("CFG_IS_STABLE"), "true")
}
