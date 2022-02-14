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

#![deny(clippy::pedantic)]
#![deny(warnings)]

use std::env;
use std::fs;

use base::error::{Error, ErrorKind, Result};

enum FileScope<'a> {
    IfConstruct { name: &'a str },
    Root,
}

pub fn load() -> Result<()> {
    let file = fs::read_to_string("Env.vars")?;
    let file = file.trim_end().to_string();

    let lines = file
        .lines()
        .filter(|line| !line.starts_with(';') && !line.is_empty());

    let mut state = FileScope::Root;
    let enable_pgsql = matches!(env!("ENABLE_PGSQL_BACKEND"), "true");
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
                name: "ENABLE_PGSQL_BACKEND",
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
