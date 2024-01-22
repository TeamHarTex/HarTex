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

use std::fs;
use std::path::absolute;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use self::flags::BootstrapSubcommand;
use self::flags::Flags;
use self::ini::IniConfig;

pub mod flags;
pub mod ini;

pub struct Config {
    pub bypass_fs_lock: bool,
    pub config_path: Option<PathBuf>,
    pub output_dir: PathBuf,
    pub root: PathBuf,
    pub subcommand: BootstrapSubcommand,
}

impl Config {
    pub fn parse_from_args(args: &[String]) -> Self {
        Self::parse_from_args_inner(args, |path| {
            let content = fs::read_to_string(path)
                .expect(&format!("configuration file not found: {}", path.display()));

            serde_ini::from_str(&content).unwrap_or_else(|error| {
                eprintln!(
                    "failed to parse configuration file {}: {error}",
                    path.display()
                );
                exit(2);
            })
        })
    }

    fn parse_from_args_inner(args: &[String], get_ini: impl Fn(&Path) -> IniConfig) -> Self {
        let flags = Flags::parse_from_args(args);
        let mut config = Self::default();

        config.bypass_fs_lock = flags.bypass_fs_lock;
        config.subcommand = flags.subcommand;

        if !config.output_dir.is_absolute() {
            config.output_dir = absolute(&config.output_dir)
                .expect("failed to resolve absolute path of output directory");
        }

        let _ = if let Some(config_path) = config.config_path.clone()
            && !config_path.exists()
        {
            config.config_path.replace(config.root.join(config_path));
            get_ini(config.config_path.as_ref().unwrap())
        } else {
            IniConfig::default()
        };

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        Self {
            bypass_fs_lock: false,
            config_path: Some(PathBuf::from("hartex.conf")),
            output_dir: PathBuf::from("build"),
            root: manifest_dir.parent().unwrap().to_owned(),
            subcommand: BootstrapSubcommand::Build,
        }
    }
}
