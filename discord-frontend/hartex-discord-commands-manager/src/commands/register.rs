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

use clap::ArgMatches;
use walkdir::WalkDir;
use hartex_discord_core::dotenvy;
use hartex_discord_core::log;

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
pub async fn register_command(matches: ArgMatches) -> hartex_discord_eyre::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    log::trace!("searching for the command specification");
    log::warn!(
        "an error will occur if this command is not ran within the discord-frontend directory"
    );

    let mut command = matches
        .get_one::<String>("command")
        .expect("unreachable code: this should never be none at all")
        .clone();

    if !command.to_ascii_lowercase().ends_with(".json") {
        command.push_str(".json");
    }

    let mut iterator = WalkDir::new("hartex-discord-commands-spec").same_file_system(true).into_iter();
    let _ = loop {
        let option = iterator.next();
        if option.is_none() {
            break None;
        }

        let entry = option.unwrap()?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        if entry.path().ends_with(&command) {
            break Some(entry);
        }
    };

    Ok(())
}
