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
use std::panic::AssertUnwindSafe;
use std::time::Duration;
use std::time::SystemTime;

use futures_util::FutureExt;
use hartex_backend_models_v2::uptime::UptimeUpdate;
use hartex_discord_core::discord::model::application::interaction::InteractionType;
use hartex_discord_core::discord::model::gateway::event::DispatchEvent;
use hartex_discord_core::discord::model::gateway::event::GatewayEvent;
use hartex_discord_core::discord::model::gateway::payload::outgoing::request_guild_members::RequestGuildMembersInfo;
use hartex_discord_core::discord::model::gateway::payload::outgoing::RequestGuildMembers;
use hartex_discord_core::discord::model::gateway::OpCode;
use hartex_discord_core::discord::model::http::interaction::{
    InteractionResponse, InteractionResponseType,
};
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::tokio::net::TcpStream;
use hartex_discord_core::tokio::spawn;
use hartex_discord_utils::CLIENT;
use hartex_log::log;
use hyper::client::conn::http1::handshake;
use hyper::header::ACCEPT;
use hyper::header::CONTENT_TYPE;
use hyper::Method;
use hyper::Request;
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;
use rdkafka::error::KafkaError;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;

/// Invoke a corresponding event callback for an event.
#[allow(clippy::cast_lossless)]
#[allow(clippy::large_futures)]
pub async fn invoke(
    event: GatewayEvent,
    shard: u8,
    producer: FutureProducer,
) -> miette::Result<()> {
    let topic = env::var("KAFKA_TOPIC_OUTBOUND_COMMUNICATION").into_diagnostic()?;

    #[allow(clippy::collapsible_match)]
    match event {
        GatewayEvent::Dispatch(seq, dispatch) => match dispatch {
            DispatchEvent::GuildCreate(guild_create) => {
                log::trace!(
                    "shard {shard} has received GUILD_CREATE payload from Discord (sequence {seq})"
                );

                let request = RequestGuildMembers {
                    d: RequestGuildMembersInfo {
                        guild_id: guild_create.id,
                        limit: Some(0),
                        nonce: None,
                        presences: Some(true),
                        query: Some(String::new()),
                        user_ids: None,
                    },
                    op: OpCode::RequestGuildMembers,
                };
                let string = serde_json::to_string(&request).into_diagnostic()?;
                if let Err((error, _)) = producer
                    .send(
                        FutureRecord::to(&topic)
                            .key(&format!("OUTBOUND_REQUEST_GUILD_MEMBERS_{shard}"))
                            .payload(&string),
                        Timeout::After(Duration::from_secs(0)),
                    )
                    .await
                {
                    println!("{:?}", Err::<(), KafkaError>(error).into_diagnostic());
                }

                Ok(())
            }
            DispatchEvent::InteractionCreate(interaction_create)
                if interaction_create.kind == InteractionType::ApplicationCommand =>
            {
                log::trace!(
                    "shard {shard} has received INTERACTION_CREATE payload from Discord (sequence {seq})"
                );

                if let Err(error) = AssertUnwindSafe(crate::interaction::application_command(
                    interaction_create.clone(),
                ))
                .catch_unwind()
                .await
                {
                    let interaction_client = CLIENT.interaction(interaction_create.application_id);
                    interaction_client
                        .create_response(
                            interaction_create.id,
                            &interaction_create.token,
                            &InteractionResponse {
                                kind: InteractionResponseType::ChannelMessageWithSource,
                                data: Some(
                                    InteractionResponseDataBuilder::new()
                                        .content("This command encountered a critical error.")
                                        .build(),
                                ),
                            },
                        )
                        .await
                        .into_diagnostic()?;
                    log::error!(
                        "interaction command panicked: {}",
                        error.downcast_ref::<String>().unwrap_or(&String::new())
                    );
                }

                Ok(())
            }
            DispatchEvent::Ready(ready) => {
                log::info!(
                    "{}#{} (shard {shard}) has received READY payload from Discord (gateway v{}) (sequence {seq})",
                    ready.user.name,
                    ready.user.discriminator,
                    ready.version
                );

                let api_domain = env::var("API_DOMAIN").into_diagnostic()?;
                let uri = format!("http://{}/api/v2/stats/uptime", api_domain.clone());
                let now = SystemTime::now();
                let duration = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .into_diagnostic()?;

                let stream = TcpStream::connect(api_domain).await.into_diagnostic()?;
                let (mut sender, connection) =
                    handshake(TokioIo::new(stream)).await.into_diagnostic()?;

                spawn(async move {
                    if let Err(err) = connection.await {
                        log::error!("TCP connection failed: {:?}", err);
                    }
                });

                log::debug!("sending a request to {}", &uri);

                let query = UptimeUpdate::new("HarTex Nightly", duration.as_secs() as u128);
                let request = Request::builder()
                    .uri(uri)
                    .method(Method::PATCH)
                    .header(ACCEPT, "application/json")
                    .header(CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(&query).into_diagnostic()?)
                    .into_diagnostic()?;

                sender.send_request(request).await.into_diagnostic()?;

                Ok(())
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}
