/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Deserialization Infrastructure for `hartex.conf`

use serde::Deserialize;

/// Root object of the configuration.
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Deserialize)]
pub struct IniConfig {
    /// Customized build options, if any.
    pub build: Option<IniBuild>,
    /// Customized Rust-related build options, if any.
    pub rust: Option<IniRust>,
}

/// Build options object.
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Deserialize)]
pub struct IniBuild {
    /// A custom output directory for the built artifacts, if any. Defaults to `build` if
    /// unspecified.
    pub output_dir: Option<String>,
}

/// Rust options object.
#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize)]
pub struct IniRust {
    /// The number of code generation units to use.
    #[serde(default = "rust_default_codegen_units")]
    pub codegen_units: u32,
    /// Whether to include debug information.
    #[serde(default = "rust_default_debug")]
    pub debug: bool,
    /// Optimization level.
    #[serde(default = "rust_default_opt_level")]
    pub opt_level: u32,
    /// Number of parallel threads.
    #[serde(default = "rust_default_parallel_threads")]
    pub parallel_threads: u32,
}

impl Default for IniRust {
    fn default() -> Self {
        Self {
            codegen_units: rust_default_codegen_units(),
            debug: rust_default_debug(),
            opt_level: rust_default_opt_level(),
            parallel_threads: rust_default_parallel_threads(),
        }
    }
}

/// Default number of code generation units to use.
///
/// Set to `1` to not miss any potential compiler optimizations that may arise as the
/// number of code generation units increases.
#[must_use]
fn rust_default_codegen_units() -> u32 {
    1
}

/// Default value for the inclusion of debug information.
///
/// Set to `true` for enhanced debugging perposes.
#[must_use]
fn rust_default_debug() -> bool {
    true
}

/// Default optimization level.
///
/// Set to `3` for maximum optimization.
#[must_use]
fn rust_default_opt_level() -> u32 {
    3
}

/// Default number of parallel threads to use.
///
/// Set to `8`.
#[must_use]
fn rust_default_parallel_threads() -> u32 {
    8
}
