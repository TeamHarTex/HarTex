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

use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_eyre::eyre::Report;
use hartex_log::log;
use hyper::body::HttpBody;
use hyper::header::ACCEPT;
use hyper::header::USER_AGENT;
use hyper::Client;
use hyper::Method;
use hyper::Request;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(name = "uptime")]
pub struct Uptime;

impl Command for Uptime {
    async fn execute(&self, _: Interaction) -> hartex_eyre::Result<()> {
        let client = Client::builder().build_http::<String>();
        let api_domain = env::var("API_DOMAIN")?;
        let uri = format!("http://{api_domain}/api/v1/uptime");

        // todo JSON
        let request = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(ACCEPT, "application/json")
            .header(
                USER_AGENT,
                "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.1.0) DiscordFrontend"
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

        Ok(())
    }
}
