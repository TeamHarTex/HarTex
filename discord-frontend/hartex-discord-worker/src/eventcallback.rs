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

use std::{env, panic::AssertUnwindSafe, time::SystemTime};

use futures_util::FutureExt;
use hartex_backend_models::uptime::UptimeUpdate;
use hartex_discord_core::{
    discord::{
        cache::DefaultInMemoryCache,
        model::{
            application::interaction::InteractionType,
            gateway::{
                event::{DispatchEvent, Event, GatewayEvent},
                payload::incoming::GuildCreate,
            },
        },
    },
    tokio,
    tokio::{net::TcpStream, sync::mpsc::Sender},
};
use hartex_discord_grpc_protos::gateway::{
    GatewayClientEventResponse, GatewayClientEventResponseStatus,
};
use hartex_discord_utils::CLIENT;
use hartex_localization_core::{LOCALIZATION_HOLDER, Localizer};
use hyper::{
    Method, Request,
    client::conn::http1::handshake,
    header::{ACCEPT, CONTENT_TYPE},
};
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;
use tonic::Status;

use crate::errorhandler::ErrorPayload;

/// Invoke a corresponding event callback for an event.
#[allow(clippy::cast_lossless)]
#[allow(clippy::large_futures)]
#[allow(clippy::too_many_lines)]
pub async fn invoke(
    event: GatewayEvent,
    shard: u64,
    cache: &DefaultInMemoryCache,
    response_tx: &Sender<Result<GatewayClientEventResponse, Status>>,
) -> miette::Result<()> {
    let flattened_event = Event::from(event.clone());
    cache.update(&flattened_event);

    #[allow(clippy::collapsible_match)]
    match event {
        GatewayEvent::Dispatch(seq, dispatch) => match dispatch {
            DispatchEvent::GuildCreate(deref!(GuildCreate::Available(ref guild_create))) => {
                hartex_tracing::trace!(
                    "shard {shard} has received GUILD_CREATE payload from Discord (sequence {seq})"
                );

                response_tx
                    .send(Ok(GatewayClientEventResponse {
                        status: GatewayClientEventResponseStatus::StatusRequestGuildMembers.into(),
                        guild_id: Some(guild_create.id.to_string()),
                    }))
                    .await
                    .into_diagnostic()
            }
            DispatchEvent::InteractionCreate(interaction_create)
                if interaction_create.kind == InteractionType::ApplicationCommand =>
            {
                hartex_tracing::trace!(
                    "shard {shard} has received INTERACTION_CREATE payload from Discord (sequence {seq})"
                );

                let interaction_client = CLIENT.interaction(interaction_create.application_id);

                let cloned = interaction_create.clone();
                let locale = cloned.locale.as_deref().unwrap_or("en-GB");
                let localizer = Localizer::new(&LOCALIZATION_HOLDER, locale);

                if let Err(error) = AssertUnwindSafe(crate::interaction::application_command(
                    interaction_create.clone(),
                    &interaction_client,
                    &localizer,
                    cache,
                ))
                .catch_unwind()
                .await
                {
                    crate::errorhandler::handle_interaction_error(
                        ErrorPayload::Panic(
                            error
                                .downcast_ref::<String>()
                                .unwrap_or(&String::new())
                                .clone(),
                        ),
                        interaction_create,
                        &interaction_client,
                        &localizer,
                    )
                    .await;
                }

                Ok(())
            }
            DispatchEvent::Ready(ready) => {
                hartex_tracing::info!(
                    "{ready.user.name}#{ready.user.discriminator} (shard {shard}) has received READY payload from Discord (gateway v{ready.version}) (sequence {seq})",
                );

                let api_domain = env::var("API_DOMAIN").into_diagnostic()?;
                let uri =
                    hartex_tracing::format!("http://{api_domain.clone()}/api/v1/stats/uptime");
                let now = SystemTime::now();
                let duration = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .into_diagnostic()?;

                let stream = TcpStream::connect(api_domain).await.into_diagnostic()?;
                let (mut sender, connection) =
                    handshake(TokioIo::new(stream)).await.into_diagnostic()?;

                tokio::spawn(async move {
                    if let Err(err) = connection.await {
                        hartex_tracing::error!("TCP connection failed: {err:?}");
                    }
                });

                hartex_tracing::debug!("sending a request to {&uri}");

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
