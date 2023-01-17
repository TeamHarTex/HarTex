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

use std::fs::File;
use std::io::Read;

use hartex_discord_core::discord::model::CommandManagerCommand;
use hartex_discord_core::log;
use hartex_discord_eyre::eyre::Report;
use walkdir::WalkDir;

pub fn list_command() -> hartex_discord_eyre::Result<()> {
    log::trace!("reading specification directory");
    log::warn!(
        "an error will occur if this command is not ran within the discord-frontend directory"
    );
    for result in WalkDir::new("hartex-discord-commands-spec").same_file_system(true) {
        let entry = result?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        let Some(ext_osstr) = entry.path().extension() else {
            continue;
        };

        if ext_osstr.to_str().unwrap() != "json" {
            continue;
        }

        let mut buffer = String::new();
        File::open(entry.path())?.read_to_string(&mut buffer)?;

        let result = serde_json::from_str::<CommandManagerCommand>(&buffer);
        let command = match result {
            Ok(command) => command,
            Err(error) => {
                log::warn!(
                    "deserialization failed for file: {}",
                    entry.path().to_str().unwrap()
                );
                println!("{:?}", Report::new(error));
                log::warn!("skipping file due to above error");

                continue;
            }
        };

        println!("{command}");
    }

    Ok(())
}
