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

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

use bootstrap::config::Config;
use fd_lock::RwLock;

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
                lock.write(&process::id().to_string().as_ref())
                    .expect("failed to write process id to lockfile");
                lock
            }
            error => {
                drop(error);
                println!("WARNING: build directory locked by process {process_id}");

                let mut lock = lock.write().expect("failed to get write lock on lockfile");
                lock.write(&process::id().to_string().as_ref())
                    .expect("failed to write process id to lockfile");
                lock
            }
        }
    }
}
