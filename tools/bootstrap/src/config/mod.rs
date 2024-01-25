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
use std::mem;
use std::path::absolute;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use self::flags::BootstrapSubcommand;
use self::flags::Flags;
use self::ini::IniBuild;
use self::ini::IniConfig;

pub mod flags;
pub mod ini;

pub struct Config {
    pub bypass_fs_lock: bool,
    pub config_path: Option<PathBuf>,
    pub output_json: bool,
    pub root: PathBuf,
    pub subcommand: BootstrapSubcommand,
    pub subcommand_args: Vec<String>,

    pub output_dir: PathBuf,

    pub codegen_units: u32,
    pub debug: bool,
    pub opt_level: u32,
    pub parallel_threads: u32,
}

impl Config {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn parse_from_args(args: &[String]) -> Self {
        Self::parse_from_args_inner(args, |path| {
            let content = fs::read_to_string(path)
                .expect(&format!("configuration file not found: {}", path.display()));

            toml::from_str(&content).unwrap_or_else(|error| {
                eprintln!(
                    "failed to parse configuration file {}: {error}",
                    path.display()
                );
                exit(2);
            })
        })
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    fn parse_from_args_inner(args: &[String], get_ini: impl Fn(&Path) -> IniConfig) -> Self {
        let mut flags = Flags::parse_from_args(args);
        let mut config = Self::default();

        config.bypass_fs_lock = flags.bypass_fs_lock;
        config.output_json = flags.json;
        config.subcommand = flags.subcommand;
        config.subcommand_args = mem::take(&mut flags.subcommand_args);

        if config.config_path.is_none() {
            config.config_path.replace(config.root.join("hartex.conf"));
        }

        let ini = if let Some(path) = &config.config_path
            && path.exists()
        {
            get_ini(config.config_path.as_ref().unwrap())
        } else {
            config.config_path = None;
            IniConfig::default()
        };

        let IniBuild { output_dir } = ini.build.unwrap_or_default();

        config.output_dir = output_dir.map_or(PathBuf::from("build"), PathBuf::from);

        if !config.output_dir.is_absolute() {
            config.output_dir = absolute(&config.output_dir)
                .expect("failed to resolve absolute path of output directory");
        }

        if let Some(rust) = ini.rust {
            config.codegen_units = rust.codegen_units;
            config.debug = rust.debug;
            config.opt_level = rust.opt_level;
            config.parallel_threads = rust.parallel_threads;
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        Self {
            bypass_fs_lock: false,
            config_path: None,
            output_json: false,
            root: manifest_dir.parent().unwrap().parent().unwrap().to_owned(),
            subcommand: BootstrapSubcommand::Build,
            subcommand_args: Vec::new(),

            output_dir: PathBuf::from("build"),

            codegen_units: 1,
            debug: true,
            opt_level: 3,
            parallel_threads: 8,
        }
    }
}
