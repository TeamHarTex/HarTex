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

use std::env;
use std::fs::File;
use std::io::Read;

use clap::ArgMatches;
use hartex_discord_core::dotenvy;
use hartex_discord_core::tokio::net::TcpStream;
use hartex_discord_core::tokio::task::spawn;
use hartex_log::log;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Buf;
use hyper::body::Bytes;
use hyper::client::conn::http1::handshake;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::CONTENT_LENGTH;
use hyper::header::CONTENT_TYPE;
use hyper::header::USER_AGENT;
use hyper::Method;
use hyper::Request;
use hyper::Uri;
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;
use miette::Report;
use minify::json::minify;
use walkdir::WalkDir;

/// Patch a command.
#[allow(clippy::module_name_repetitions)]
pub async fn patch_command(matches: ArgMatches) -> miette::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    log::trace!("searching for the command specification");
    log::warn!(
        "an error will occur if this command is not ran within the discord-frontend directory"
    );

    let mut command = matches.get_one::<String>("command").unwrap().clone();
    let command_id = matches.get_one::<String>("command-id").unwrap().clone();

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

        let entry = option.unwrap().into_diagnostic()?;
        if entry.metadata().into_diagnostic()?.is_dir() {
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

    let mut file = File::open(entry_option.unwrap().path()).into_diagnostic()?;
    let mut json = String::new();
    file.read_to_string(&mut json).into_diagnostic()?;

    log::trace!("making tcp connection");
    let uri = "https://discord.com".parse::<Uri>().into_diagnostic()?;
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(443);

    let stream = TcpStream::connect(format!("{host}:{port}"))
        .await
        .into_diagnostic()?;
    let (mut sender, connection) = handshake(TokioIo::new(stream))
        .await
        .into_diagnostic()?;

    spawn(async move {
        if let Err(err) = connection.await {
            log::error!("TCP connection failed: {:?}", err);
        }
    });

    let application_id = env::var("APPLICATION_ID").into_diagnostic()?;

    let mut token = env::var("BOT_TOKEN").into_diagnostic()?;
    if !token.starts_with("Bot ") {
        token.insert_str(0, "Bot ");
    }

    let bytes = Bytes::from(minify(&json));

    log::trace!("sending request with body {:?}", bytes.clone());
    let request = Request::builder()
        .uri(format!(
            "https://discord.com/api/v10/applications/{application_id}/commands/{command_id}"
        ))
        .method(Method::PATCH)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_LENGTH, bytes.len())
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.6.0) CommandsManager",
        )
        .body(Full::<Bytes>::new(bytes))
        .into_diagnostic()?;
    let result = sender.send_request(request).await.into_diagnostic()?;

    log::info!("received response with status {}", result.status());
    let body = result.into_body().collect().await.into_diagnostic()?.aggregate();
    let mut string = String::new();
    body.reader().read_to_string(&mut string).into_diagnostic()?;
    log::info!("response body: {string:?}");

    Ok(())
}
