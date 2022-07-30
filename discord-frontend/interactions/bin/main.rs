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

use base::error::Result;
use clap::Command;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<()> {
    let command = clap::command!().subcommand(Command::new("create-cmd").args(&[
        clap::arg!(<COMMAND_NAME> "The name of the command"),
        clap::arg!(<COMMAND_TYPE> "The type of the command to create"),
    ]));
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("create-cmd", _)) => todo!(),
        _ => (),
    }

    Ok(())
}
