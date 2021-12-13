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
 * with HarTex; if not, If not, see <https://www.gnu.org/licenses/>.
 */

//! # `hartex_core` - The `HarTex` Core Library
//!
//! The `hartex_core` library contains the core functionality for the `HarTex` Discord bot.
//!
//! ## Features
//!
//! - `twilight-bundled`: bundles most of the `twilight` ecosystem of crates with the library,
//!                       removes the need to include the dependencies repeatedly across the
//!                       `HarTex` crates.
//!
//! - `tracing-bundled`: bundles tracing, a logging library for use within the `HarTex` crates.

#![deny(clippy::pedantic, warnings, unsafe_code)]

pub use ctrlc;

#[cfg(feature = "twilight-bundled")]
pub mod discord;
pub mod error;
pub mod events;
#[cfg(feature = "tracing-bundled")]
pub mod logging;
pub mod time;

/// # Static `HARTEX_BUILD`
///
/// Represents the current version and build of `HarTex` Discord bot.
pub static HARTEX_BUILD: &str = "Version 1.25.0-nightly-13-12-2021 (Build 636)";

/// # Static `IS_STABLE`
///
/// Determines whether this version of the bot is stable.
pub static STABLE: bool = false;
