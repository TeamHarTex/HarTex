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

use base::error::{Error, ErrorKind, Result};
use std::fmt::Display;
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

pub(crate) fn prompt<T>(question: &str) -> Result<T>
where
    T: Default + Display + FromStr,
{
    let default = T::default();

    println!("{question} [default: {default}]");
    let _ = io::stdout().flush();
    let input = readln()?;

    println!();

    Ok(input.parse().unwrap_or(default))
}

fn readln() -> Result<String> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();
    let lines = lines.next().transpose()?;

    lines.ok_or(Error {
        kind: ErrorKind::AnyError {
            description: "failed to read lines from stdin",
        },
    })
}
