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

//! # Command Line Handler

use clap::ArgMatches;

use crate::commands;

/// Handle the command line with argument matches.
pub async fn handle(matches: ArgMatches) -> hartex_eyre::Result<()> {
    match matches.subcommand() {
        Some(("list-from-discord", subcommand_matches)) => {
            commands::list_from_discord::list_from_discord_command(subcommand_matches.clone()).await
        }
        Some(("list-from-fs", _)) => commands::list_from_fs::list_from_fs_command(),
        Some(("patch", subcommand_matches)) => {
            commands::patch::patch_command(subcommand_matches.clone()).await
        }
        Some(("register", subcommand_matches)) => {
            commands::register::register_command(subcommand_matches.clone()).await
        }
        Some(("unregister", subcommand_matches)) => {
            commands::unregister::unregister_command(subcommand_matches.clone()).await
        }
        _ => Ok(()),
    }
}
