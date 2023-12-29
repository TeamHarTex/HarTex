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

//! # Command Line Handling

use clap::ArgMatches;

use crate::commands;

/// Handle a command line command with the matches.
pub fn handle(matches: &ArgMatches) -> miette::Result<()> {
    match matches.subcommand() {
        Some(("build", subcommand_matches)) => {
            commands::build::build_command(subcommand_matches)
        }
        Some(("clean", subcommand_mateches)) => {
            commands::clean::clean_command(subcommand_mateches)
        }
        Some(("lint", subcommand_matches)) => {
            commands::lint::lint_command(subcommand_matches)
        }
        Some(("test", subcommand_matches)) => {
            commands::test::test_command(subcommand_matches)
        }
        Some(("update", subcommand_matches)) => {
            commands::update::update_command(subcommand_matches)
        }
        _ => Ok(()),
    }
}
