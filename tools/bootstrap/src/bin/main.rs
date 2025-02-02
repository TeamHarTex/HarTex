/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

use bootstrap::build::Build;
use bootstrap::config::Config;
use bootstrap::config::flags::BootstrapSubcommand;
use owo_colors::OwoColorize;
use fd_lock::RwLock;

/// Entry point to the bootstrap binary, invoked by x.py.
#[allow(clippy::unused_io_amount)]
pub fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let config = Config::parse_from_args(&args);

    let mut lock;
    let _lock_guard;

    if !config.bypass_fs_lock {
        let lockfile = config.output_dir.join("lockfile");
        let process_id = if let Ok(contents) = fs::read_to_string(&lockfile) {
            contents
        } else {
            String::new()
        };

        lock = RwLock::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&lockfile)
                .expect("failed to create or open lockfile"),
        );

        _lock_guard = match lock.try_write() {
            Ok(mut lock) => {
                lock.write(process::id().to_string().as_ref())
                    .expect("failed to write process id to lockfile");
                lock
            }
            error => {
                drop(error);
                println!("{} build directory locked by process {process_id}", "warning:".yellow().bold());

                let mut lock = lock.write().expect("failed to get write lock on lockfile");
                lock.write(process::id().to_string().as_ref())
                    .expect("failed to write process id to lockfile");
                lock
            }
        }
    }

    if config.config_path.is_none() && !matches!(config.subcommand, BootstrapSubcommand::Setup) {
        println!("{} no `hartex.conf` configuration file is found, using default configuration", "warning:".yellow().bold());
        println!(
            "{} consider running `./x.py setup` or copying `hartex.example.conf` by running `cp hartex.example.conf hartex.conf`",
            "help:".bold(),
        )
    }

    Build::new(config).build();
}
