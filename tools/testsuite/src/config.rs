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

use std::path::PathBuf;

use crate::flags::Flags;

#[derive(Clone)]
pub struct Config {
    pub build_dir: PathBuf,
    pub root: PathBuf,
    pub ui: bool,
}

impl Config {
    pub fn from_flags(flags: Flags) -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        Self {
            build_dir: flags.build_dir,
            root: manifest_dir.parent().unwrap().parent().unwrap().to_owned(),
            ui: flags.ui,
        }
    }
}
