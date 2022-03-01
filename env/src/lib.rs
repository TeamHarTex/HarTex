/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The HarTex Environment Manipulation Library
//!
//! The HarTex Environment Manipulation Library is a utility library for interacting with the
//! system environment (for instance, loading and retrieving environment variables).
//!
//! This library provides a helper function to load environment variables from an `Env.vars` file,
//! which is a custom file format specifically designed for HarTex. Its syntax to some extent
//! mimics a C header file, and it seemed quite fitting for the implementation and usage for
//! HarTex.
//!
//! After loading the environment variables, the helper function then continues to set the
//! configured variables for the environment of the current running process, where they only last
//! for the lifetime of the process. Once the process is terminated, the environment variables for
//! that running process  are discarded.

#![deny(clippy::pedantic)]
#![deny(warnings)]

use std::env;
use std::fs;

use base::error::{Error, ErrorKind, Result};

enum FileScope<'a> {
    IfConstruct { name: &'a str },
    Root,
}

/// Loads the environment variables from an `Env.vars` file and sets the configured variables for
/// the environment of the current running process.
///
/// ## Errors
///
/// This function returns [`ErrorKind::IoError`] if the contents of the `Env.vars` file could not
/// be read due to some I/O errors, for example the file does not exist, or permission is denied
/// for the file.
///
/// This function returns [`ErrorKind::EnvFileError`] if the contents of the `Env.vars` file are
/// invalid or has invalid syntax and cannot be further processed.
#[allow(clippy::missing_panics_doc)] // this function never panics
pub fn load() -> Result<()> {
    let file = fs::read_to_string("env.vars")?;
    let file = file.trim_end().to_string();

    let lines = file
        .lines()
        .filter(|line| !line.starts_with(';') && !line.is_empty());

    let mut state = FileScope::Root;
    let enable_pgsql = matches!(env!("ENABLE_PGSQL_CACHE_BACKEND"), "true");
    for line in lines {
        match state {
            FileScope::Root => {
                if line.starts_with("if defined") {
                    let split = line.split(' ').collect::<Vec<_>>();
                    let name = split.get(3);
                    if name.is_none() {
                        return Err(Error::from(ErrorKind::EnvFileError {
                            description: "missing identifier to check for definition",
                        }));
                    }

                    state = FileScope::IfConstruct {
                        name: <&str>::clone(name.unwrap()),
                    };
                    continue;
                }

                if line.starts_with("define") {
                    let split = line.split(' ').collect::<Vec<_>>();
                    env::set_var(split[1], split[2]);
                }
            }
            FileScope::IfConstruct {
                name: "ENABLE_PGSQL_CACHE_BACKEND",
            } => {
                if enable_pgsql && line.starts_with("define ") {
                    let split = line.split(' ').collect::<Vec<_>>();
                    env::set_var(split[1], split[2]);
                }

                if line.starts_with("endif") {
                    state = FileScope::Root;
                }
            }
            FileScope::IfConstruct { .. } => (),
        }
    }

    Ok(())
}
