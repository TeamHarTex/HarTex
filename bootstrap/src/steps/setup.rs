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
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use crate::builder::Builder;
use crate::builder::RunConfig;
use crate::builder::Step;

pub enum SetupProfile {
    All,
    Distribution,
    Localization,
    None,
}

impl Step for SetupProfile {
    type Output = ();

    fn run_config(run: RunConfig<'_>) {
        let path = &run
            .builder
            .config
            .config_path
            .clone()
            .unwrap_or(PathBuf::from("hartex.conf"));
        if path.exists() {
            eprintln!();
            eprintln!(
                "WARN: a configuration file already exists at {}",
                path.canonicalize()
                    .expect("failed to canonicalize path")
                    .display()
            );

            match question_bool("Do you wish to override the existing configuration, to allow the setup process to continue?", false) {
                Ok(true) => fs::remove_file(path).expect("failed to remove file"),
                _ => {
                    println!("Setup cancelled. Exiting.");
                    exit(1);
                }
            }
        }
    }

    fn run(self, _: &Builder<'_>) -> Self::Output {}
}

fn question_bool(prompt: &str, default: bool) -> io::Result<bool> {
    let default_text = if default { "(Y/n)" } else { "(y/N)" };
    writeln!(io::stdout().lock(), "{prompt} {default_text}")?;

    let _ = io::stdout().flush()?;
    let input = readln()?;

    writeln!(io::stdout().lock())?;

    if input.is_empty() {
        Ok(default)
    } else {
        match &*input.to_lowercase() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => Ok(default),
        }
    }
}

fn readln() -> io::Result<String> {
    let stdin = io::stdin();
    let locked = stdin.lock();
    let mut lines = locked.lines();

    lines.next().transpose()?.ok_or(io::Error::new(
        io::ErrorKind::Uncategorized,
        "failed to read from stdin",
    ))
}
