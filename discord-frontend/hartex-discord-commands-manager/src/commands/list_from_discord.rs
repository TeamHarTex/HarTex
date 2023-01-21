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
use std::str;

use clap::ArgMatches;
use hartex_discord_core::discord::model::application::command::Command;
use hartex_discord_core::dotenvy;
use hartex_discord_core::log;
use hartex_discord_eyre::eyre::Report;
use hyper::body::HttpBody;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::USER_AGENT;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use hyper_trust_dns::TrustDnsResolver;
use owo_colors::OwoColorize;

pub async fn list_from_discord_command(matches: ArgMatches) -> hartex_discord_eyre::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    let client =
        Client::builder().build(TrustDnsResolver::default().into_native_tls_https_connector());

    let application_id = env::var("APPLICATION_ID")?;

    let mut token = env::var("BOT_TOKEN")?;
    if !token.starts_with("Bot ") {
        token.insert_str(0, "Bot ");
    }

    let mut uri = format!("https://discord.com/api/v10/applications/{application_id}/commands");
    if let Some(flag) = matches.get_one::<bool>("with-localizations") && *flag {
        uri.push_str("?with_localizations=true");
    }

    let request = Request::builder()
        .uri(uri)
        .method(Method::GET)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, token)
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.1.0) CommandsManager",
        )
        .body(String::new())?;
    let mut response = client.request(request).await?;
    let mut full = String::new();
    while let Some(result) = response.body_mut().data().await {
        full.push_str(str::from_utf8(&result?)?);
    }
    if !response.status().is_success() {
        log::error!("unsuccessful HTTP request, response: {full}");

        return Err(Report::msg(format!(
            "unsuccessful HTTP request, with status code {}",
            response.status()
        )));
    }

    let commands = serde_json::from_str::<Vec<Command>>(&full)?;
    for command in commands {
        println!();
        println!(
            "{}{}",
            "Command ID: ".bold(),
            command.id.unwrap().bright_cyan()
        );
        println!("{}{}", "Command Name: ".bold(), command.name.bright_cyan());
        println!(
            "{}{}",
            "Command Description: ".bold(),
            command.description.bright_cyan()
        );
    }

    Ok(())
}
