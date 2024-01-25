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

use serde::Deserialize;

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Deserialize)]
pub struct IniConfig {
    pub build: Option<IniBuild>,
    pub rust: Option<IniRust>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Deserialize)]
pub struct IniBuild {
    pub output_dir: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize)]
pub struct IniRust {
    #[serde(default = "rust_default_codegen_units")]
    pub codegen_units: u32,
    #[serde(default = "rust_default_debug")]
    pub debug: bool,
    #[serde(default = "rust_default_opt_level")]
    pub opt_level: u32,
    #[serde(default = "rust_default_parallel_threads")]
    pub parallel_threads: u32,
}

impl Default for IniRust {
    fn default() -> Self {
        Self {
            codegen_units: 1,
            debug: true,
            opt_level: 3,
            parallel_threads: 8,
        }
    }
}

#[must_use]
fn rust_default_codegen_units() -> u32 {
    1
}

#[must_use]
fn rust_default_debug() -> bool {
    true
}

#[must_use]
fn rust_default_opt_level() -> u32 {
    3
}

#[must_use]
fn rust_default_parallel_threads() -> u32 {
    8
}
