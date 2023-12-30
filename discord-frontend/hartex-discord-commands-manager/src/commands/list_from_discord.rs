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
use std::io::Read;

use clap::ArgMatches;
use hartex_discord_core::dotenvy;
use hartex_discord_core::tokio::task::spawn;
use hartex_discord_utils::hyper::tls_stream;
use hartex_log::log;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Buf;
use hyper::body::Bytes;
use hyper::client::conn::http1::handshake;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::CONTENT_LENGTH;
use hyper::header::HOST;
use hyper::header::USER_AGENT;
use hyper::Method;
use hyper::Request;
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;

use crate::model::command::CommandManagerCommand;

/// List commands from discord.
#[allow(clippy::module_name_repetitions)]
pub async fn list_from_discord_command(matches: ArgMatches) -> miette::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    let application_id = env::var("APPLICATION_ID").into_diagnostic()?;

    let mut token = env::var("BOT_TOKEN").into_diagnostic()?;
    if !token.starts_with("Bot ") {
        token.insert_str(0, "Bot ");
    }

    log::trace!("making tcp connection");
    let stream = tls_stream().await?;
    let (mut sender, connection) = handshake(TokioIo::new(stream)).await.into_diagnostic()?;

    spawn(async move {
        if let Err(err) = connection.await {
            log::error!("TCP connection failed: {:?}", err);
        }
    });

    let mut uri = format!("/api/v10/applications/{application_id}/commands");
    if matches.get_flag("with-localizations") {
        uri.push_str("?with_localizations=true");
    }

    log::trace!("sending request");
    let request = Request::builder()
        .uri(uri)
        .method(Method::GET)
        .header(HOST, "discord.com")
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, token)
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.6.0) CommandsManager",
        )
        .header(CONTENT_LENGTH, 0)
        .body(Empty::<Bytes>::new())
        .into_diagnostic()?;
    let result = sender.send_request(request).await.into_diagnostic()?;
    log::info!("received response with status {}", result.status());

    if !result.status().is_success() {
        let body = result
            .into_body()
            .collect()
            .await
            .into_diagnostic()?
            .aggregate();
        let mut string = String::new();
        body.reader()
            .read_to_string(&mut string)
            .into_diagnostic()?;
        log::info!("response body: {string:?}");

        return Ok(());
    }

    let body = result
        .into_body()
        .collect()
        .await
        .into_diagnostic()?
        .aggregate();

    let commands: Vec<CommandManagerCommand> =
        serde_json::from_reader(body.reader()).into_diagnostic()?;

    for command in commands {
        println!("{command}");
    }

    Ok(())
}
