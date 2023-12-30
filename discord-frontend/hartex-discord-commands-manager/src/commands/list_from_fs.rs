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

use std::fs::File;
use std::io::Read;

use hartex_discord_core::dotenvy;
use hartex_log::log;
use miette::IntoDiagnostic;
use walkdir::WalkDir;

use crate::model::command::CommandManagerCommand;

/// List commands from filesystem.
#[allow(clippy::module_name_repetitions)]
pub fn list_from_fs_command() -> miette::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    log::trace!("reading specification directory");
    log::warn!(
        "an error will occur if this command is not ran within the discord-frontend directory"
    );
    for result in WalkDir::new("hartex-discord-commands-spec").same_file_system(true) {
        let entry = result.into_diagnostic()?;
        if entry.metadata().into_diagnostic()?.is_dir() {
            continue;
        }

        let Some(ext_osstr) = entry.path().extension() else {
            continue;
        };

        if ext_osstr.to_str().unwrap() != "json" {
            continue;
        }

        let mut buffer = String::new();
        File::open(entry.path())
            .into_diagnostic()?
            .read_to_string(&mut buffer)
            .into_diagnostic()?;

        let result = serde_json::from_str::<CommandManagerCommand>(&buffer);
        let command = match result {
            Ok(command) => command,
            Err(error) => {
                log::warn!(
                    "deserialization failed for file: {}",
                    entry.path().to_str().unwrap()
                );
                println!(
                    "{:?}",
                    Err::<(), serde_json::Error>(error).into_diagnostic()
                );
                log::warn!("skipping file due to above error");

                continue;
            }
        };

        println!("{command}");
    }

    Ok(())
}
