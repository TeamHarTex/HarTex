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
#![deny(warnings)]

use clap::{Arg, ArgAction, Command};
use hartex_discord_core::log;
use hartex_discord_core::tokio;

mod cmdline;
mod commands;
mod model;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_discord_eyre::Result<()> {
    hartex_discord_eyre::initialize()?;
    log::initialize();

    let command = Command::new("cmdmgr")
        .subcommand(Command::new("list").about("Lists commands registered with Discord."))
        .subcommand(
            Command::new("register")
                .about("Registers a command with Discord.")
                .long_about(
                    "Use the -u/--update flag to specify specifically to patch the command.",
                )
                .arg(
                    Arg::new("update")
                        .short('u')
                        .long("update")
                        .action(ArgAction::Set)
                        .num_args(0),
                )
                .arg(
                    Arg::new("command")
                        .num_args(1)
                        .required(true)
                ),
        );

    let matches = command.get_matches();

    cmdline::handle(matches).await?;

    Ok(())
}
