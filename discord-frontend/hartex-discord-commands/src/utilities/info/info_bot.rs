/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # The Info Bot Subcommand
//!
//! This command returns latency and uptime information about the bot.

use std::{env, time::SystemTime};

use hartex_async_lazy::LazyResult;
use hartex_backend_models::{Response, uptime::UptimeResponse};
use hartex_discord_commands_core::context::CommandContext;
use hartex_discord_core::{
    discord::{
        model::application::interaction::application_command::CommandDataOption,
        util::builder::embed::{EmbedBuilder, EmbedFieldBuilder},
    },
    tokio::{net::TcpStream, task::spawn},
};
use hartex_discord_utils::{interaction::embed_response, markdown::MarkdownStyle};
use http_body_util::{BodyExt, Empty};
use hyper::{
    Method, Request,
    body::{Buf, Bytes},
    client::conn::http1::handshake,
    header::ACCEPT,
};
use hyper_util::rt::TokioIo;
use miette::{IntoDiagnostic, Report};

// TODO: this needs to be changed so initialization could be called again if previous calls fail
static START_TIMESTAMP: LazyResult<u128, Report> = LazyResult::new(|| {
    Box::pin(async {
        let api_domain = env::var("API_DOMAIN").into_diagnostic()?;
        let uri = hartex_tracing::format!(
            "http://{api_domain.clone()}/api/v1/stats/uptime?component=HarTex%20Nightly"
        );
        // let now = SystemTime::now();

        let stream = TcpStream::connect(api_domain).await.into_diagnostic()?;
        let (mut sender, connection) = handshake(TokioIo::new(stream)).await.into_diagnostic()?;

        spawn(async move {
            if let Err(err) = connection.await {
                hartex_tracing::error!("TCP connection failed: {err:?}");
            }
        });

        hartex_tracing::debug!("sending a request to {&uri}");

        let request = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .header(ACCEPT, "application/json")
            .body(Empty::<Bytes>::new())
            .into_diagnostic()?;

        let result = sender.send_request(request).await.into_diagnostic()?;
        hartex_tracing::debug!("deserializing result");
        let body = result.collect().await.into_diagnostic()?.aggregate();
        let response: Response<UptimeResponse, String> =
            serde_json::from_reader(body.reader()).into_diagnostic()?;

        let data = response.data();

        let data = data
            .left()
            .flatten()
            .ok_or(Report::msg("no data in response"))?;
        Ok(data.start_timestamp())
    })
});

/// Executes the `info bot` command
pub async fn execute(context: &CommandContext<'_>, _: &CommandDataOption) -> miette::Result<()> {
    let botinfo_embed_botstarted_field_name = context
        .localizer
        .utilities_plugin_botinfo_embed_botstarted_field_name()?;
    let botinfo_embed_latency_field_name = context
        .localizer
        .utilities_plugin_botinfo_embed_latency_field_name()?;
    let botinfo_embed_title = context.localizer.utilities_plugin_botinfo_embed_title()?;

    START_TIMESTAMP.force().await;

    let timestamp = START_TIMESTAMP.get().unwrap();

    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            botinfo_embed_botstarted_field_name,
            timestamp.to_string().discord_relative_timestamp(),
        ))
        .field(EmbedFieldBuilder::new(
            botinfo_embed_latency_field_name,
            timestamp.to_string().discord_inline_code(), // TODO
        ))
        .title(botinfo_embed_title)
        .validate()
        .into_diagnostic()?
        .build();

    context.create_response(embed_response(vec![embed])).await?;

    Ok(())
}
