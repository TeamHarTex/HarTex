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
 * with HarTex; if not, see <https://www.gnu.org/licenses/>.
 */

//! # `hartex_plugins` - The `HarTex` Plugins Library
//!
//! The `hartex_plugins` library implements the bot plugins.

#![deny(clippy::pedantic, warnings, unsafe_code)]
#![feature(once_cell)]

use std::lazy::SyncLazy;

use hartex_env::PluginEnv;

pub mod globadmin_only;
pub mod global;
pub mod information;
pub mod utilities;

/// # Static `PLUGIN_ENV`
///
/// Useful environment variables for various bot plugins
pub static PLUGIN_ENV: SyncLazy<PluginEnv> = SyncLazy::new(PluginEnv::get);

/// # Function `init_env`
///
/// Initializes the environment variables for later use, must be called in the "entry point" in
/// the `hartex_driver` crate for the environment variables to be usable.
pub fn init_env() {
    SyncLazy::force(&PLUGIN_ENV);
}
