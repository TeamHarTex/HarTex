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

use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

use clap::ArgMatches;
use hartex_discord_core::dotenvy;
use hartex_discord_core::log;
use hartex_discord_core::tokio::net::TcpStream;
use hartex_discord_core::tokio::task;
use hartex_discord_eyre::eyre::Report;
use http_body_util::BodyExt;
use hyper::client::conn::http1;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::CONTENT_TYPE;
use hyper::header::USER_AGENT;
use hyper::Method;
use hyper::Request;
use walkdir::WalkDir;

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
pub async fn register_command(matches: ArgMatches) -> hartex_discord_eyre::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    log::trace!("searching for the command specification");
    log::warn!(
        "an error will occur if this command is not ran within the discord-frontend directory"
    );

    let mut command = matches.get_one::<String>("command").unwrap().clone();

    if !command.to_ascii_lowercase().ends_with(".json") {
        command.push_str(".json");
    }

    let mut iterator = WalkDir::new("hartex-discord-commands-spec")
        .same_file_system(true)
        .into_iter();
    let entry_option = loop {
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

    if entry_option.is_none() {
        return Err(Report::msg(format!(
            "command file {command} cannot be found"
        )));
    }

    let mut file = File::open(entry_option.unwrap().path())?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    let stream = TcpStream::connect("discord.com:443").await?;
    let (mut sender, connection) = http1::handshake(stream).await?;
    task::spawn(async move {
        if let Err(error) = connection.await {
            println!("{:?}", Report::new(error));
        }
    });

    let application_id = env::var("APPLICATION_ID")?;
    let request = Request::builder()
        .uri(format!(
            "https://discord.com/api/v10/applications/{application_id}/commands"
        ))
        .method(Method::POST)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, env::var("BOT_TOKEN")?)
        .header(CONTENT_TYPE, "application/json")
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.1.0) CommandsManager",
        )
        .body(json)?;
    let mut response = sender.send_request(request).await?;
    if !response.status().is_success() {
        let mut full = String::new();
        while let Some(result) = response.frame().await {
            let frame = result?;
            if let Some(chunk) = frame.data_ref() {
                full.push_str(str::from_utf8(chunk)?);
            }
        }
        log::error!("unsuccessful HTTP request, response: {full}");

        return Err(Report::msg(format!(
            "unsuccessful HTTP request, with status code {}",
            response.status()
        )));
    }

    log::info!("request succeeded: {}", response.status());

    Ok(())
}
