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

#[derive(Default, Deserialize)]
pub struct IniConfig {
    pub build: Option<IniBuild>,
    pub rust: Option<IniRust>,
}

#[derive(Default, Deserialize)]
pub struct IniBuild {
    pub output_dir: Option<String>,
}

#[derive(Default, Deserialize)]
pub struct IniRust {
    pub codegen_units: Option<u32>,
    #[serde(default)]
    pub debug: bool,
    pub opt_level: Option<u32>,
    pub parallel_threads: Option<u32>,
}
