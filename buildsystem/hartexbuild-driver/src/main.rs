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

//! # Hartexbuild Binary Executable
//!
//! The main binary for the hartexbuild buildsystem.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(int_roundings)]

use std::time::Instant;

use clap::Arg;
use clap::ArgAction;
use clap::Command;

mod cmdline;
mod commands;

/// Main entry point.
pub fn main() -> miette::Result<()> {
    let command = Command::new("hartexbuild")
        .subcommand(
            Command::new("build")
                .about("Builds specified projects.")
                .arg(Arg::new("project").required(true).action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("clean")
                .about("Cleans specified projects.")
                .arg(Arg::new("project").required(true).action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("lint")
                .about("Lints specified projects.")
                .arg(Arg::new("project").required(true).action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("test")
                .about("Tests specified projects.")
                .arg(Arg::new("project").required(true).action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("update")
                .about("Updates dependencies of specified projects.")
                .arg(Arg::new("project").required(true).action(ArgAction::Append)),
        );

    let matches = command.get_matches();

    let now = Instant::now();
    cmdline::handle(&matches)?;

    let duration = now.elapsed();
    let mut seconds = duration.as_secs();

    let mut minutes = seconds.div_floor(60);
    seconds %= 60;

    let mut hours = minutes.div_floor(60);
    minutes %= 60;

    let days = hours.div_floor(24);
    hours %= 24;

    let mut output = String::from("Tasks finished in ");
    if days > 0 {
        output += &*format!("{days}d ");
    }
    if hours > 0 {
        output += &*format!("{hours}h ");
    }
    if minutes > 0 {
        output += &*format!("{minutes}m ");
    }

    output += &*format!("{seconds}s ");

    println!("{output}");

    Ok(())
}
