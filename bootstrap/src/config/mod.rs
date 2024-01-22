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

use std::path::absolute;
use std::path::PathBuf;

use self::flags::Flags;

pub mod flags;

pub struct Config {
    pub bypass_fs_lock: bool,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn parse_from_args(args: &[String]) -> Self {
        Self::parse_from_args_inner(args)
    }

    fn parse_from_args_inner(args: &[String]) -> Self {
        let flags = Flags::parse_from_args(args);
        let mut config = Self::default();

        config.bypass_fs_lock = flags.bypass_fs_lock;

        if !config.output_dir.is_absolute() {
            config.output_dir = absolute(&config.output_dir)
                .expect("failed to resolve absolute path of output directory");
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bypass_fs_lock: false,
            output_dir: PathBuf::from("build"),
        }
    }
}
