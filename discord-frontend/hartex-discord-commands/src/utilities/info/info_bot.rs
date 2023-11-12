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
use std::time::SystemTime;

use hartex_backend_models::Response;
use hartex_backend_models_v2::uptime::UptimeQuery;
use hartex_backend_models_v2::uptime::UptimeResponse;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_log::log;
use hyper::body::HttpBody;
use hyper::header::ACCEPT;
use hyper::header::USER_AGENT;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use miette::IntoDiagnostic;
use miette::Report;

use crate::localization::Localizer;

pub async fn execute(interaction: Interaction, _: CommandDataOption) -> miette::Result<()> {
    let interaction_client = CLIENT.interaction(interaction.application_id);
    let locale = interaction.locale.unwrap_or_else(|| String::from("en-GB"));
    let localizer = Localizer::new(&crate::LOCALIZATION_HOLDER, &locale);

    let client = Client::builder().build_http::<String>();
    let api_domain = env::var("API_DOMAIN").into_diagnostic()?;
    let uri = format!("http://{api_domain}/api/v2/uptime");
    let now = SystemTime::now();

    log::debug!("sending a request to {}", &uri);

    let query = UptimeQuery::new("HarTex Nightly");
    let request = Request::builder()
        .uri(uri)
        .method(Method::POST)
        .header(ACCEPT, "application/json")
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.1.0) DiscordFrontend",
        )
        .body(serde_json::to_string(&query).into_diagnostic()?)
        .into_diagnostic()?;

    let mut response = client.request(request).await.into_diagnostic()?;
    let mut full = String::new();
    while let Some(result) = response.body_mut().data().await {
        full.push_str(str::from_utf8(&result.into_diagnostic()?).into_diagnostic()?);
    }
    if !response.status().is_success() {
        log::error!("unsuccessful HTTP request, response: {full}");

        return Err(Report::msg(format!(
            "unsuccessful HTTP request, with status code {}",
            response.status()
        )));
    }

    let latency = now.elapsed().into_diagnostic()?.as_millis();

    let response = serde_json::from_str::<Response<UptimeResponse>>(&full).into_diagnostic()?;
    let data = response.data();
    let timestamp = data.start_timestamp();

    let botinfo_embed_botstarted_field_name =
        localizer.utilities_plugin_botinfo_embed_botstarted_field_name()?;
    let botinfo_embed_latency_field_name =
        localizer.utilities_plugin_botinfo_embed_latency_field_name()?;
    let botinfo_embed_title = localizer.utilities_plugin_botinfo_embed_title()?;

    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            botinfo_embed_botstarted_field_name,
            timestamp.to_string().discord_relative_timestamp(),
        ))
        .field(EmbedFieldBuilder::new(
            botinfo_embed_latency_field_name,
            latency.to_string().discord_inline_code(),
        ))
        .title(botinfo_embed_title)
        .validate()
        .into_diagnostic()?
        .build();

    interaction_client
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .embeds(vec![embed])
                        .build(),
                ),
            },
        )
        .await
        .into_diagnostic()?;

    Ok(())
}
