/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use clap::Arg;
use clap::ArgAction;
use clap::Command;

mod cmdline;
mod commands;

pub fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;

    let command = Command::new("hartexbuild")
        .subcommand(
            Command::new("build")
                .about("Builds a specified project.")
                .arg(Arg::new("project").required(true).action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("lint")
                .about("Lints a specified project.")
                .arg(Arg::new("project").required(true).action(ArgAction::Set)),
        );

    let matches = command.get_matches();

    cmdline::handle(matches)?;

    Ok(())
}
