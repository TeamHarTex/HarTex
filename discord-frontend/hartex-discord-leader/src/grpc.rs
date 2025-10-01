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

use std::{str::FromStr, sync::Arc};

use bytes::Bytes;
use futures_util::StreamExt as FutureStreamExt;
use hartex_discord_core::{
    discord::{
        gateway::{Message as GatewayMessage, Session, Shard, queue::Queue},
        model::{
            gateway::payload::outgoing::request_guild_members::RequestGuildMembersBuilder, id::Id,
        },
    },
    tokio,
    tokio::sync::{Mutex, mpsc, mpsc::error::SendError, watch::Receiver},
};
use hartex_discord_grpc_protos::gateway::{
    GatewayClientEventMessage, GatewayClientEventResponseStatus, gateway_client::GatewayClient,
};
use miette::IntoDiagnostic;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;

const CHUNK_SIZE: usize = 1024 * 1024;

/// Handle inbound AND outbound messages for a given shard.
pub async fn handle<Q>(
    shard: Arc<Mutex<Shard<Q>>>,
    client: GatewayClient<Channel>,
    terminator: Receiver<bool>,
) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    inbound(shard, client, terminator).await
}

/// Handle inbound traffic.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::match_wildcard_for_single_variants)]
async fn inbound<Q>(
    shard: Arc<Mutex<Shard<Q>>>,
    mut client: GatewayClient<Channel>,
    terminator: Receiver<bool>,
) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    let (tx, rx) = mpsc::channel(1000);

    let cloned = Arc::clone(&shard);
    tokio::spawn(async move {
        let mut shard = cloned.lock().await;
        while let Some(result) = shard.next().await
            && !terminator.has_changed().unwrap()
        {
            let shard_id = shard.id().number();
            let event_seq = shard.session().map_or(0, Session::sequence);

            match result {
                Ok(message) => {
                    let Some(bytes) = (match message {
                        // todo: handle close frame
                        GatewayMessage::Text(string) => Some(string.into_bytes()),
                        _ => None,
                    }) else {
                        continue;
                    };

                    hartex_tracing::trace!(
                        "[shard {shard.id().number()}] received payload from gateway",
                    );

                    let total_chunks = bytes.len().div_ceil(CHUNK_SIZE);
                    for (nth_chunk, chunk_data) in bytes.chunks(CHUNK_SIZE).enumerate() {
                        let message = GatewayClientEventMessage {
                            event_seq,
                            chunk_data: Bytes::from(chunk_data.to_vec()),
                            nth_chunk: nth_chunk as u32,
                            total_chunks: total_chunks as u32,
                            shard_id: u64::from(shard_id),
                        };

                        tx.send(message).await?;
                    }
                }
                Err(error) => {
                    hartex_tracing::warn!(
                        "[shard {shard.id()}] error when receiving gateway message: {error}",
                    );
                }
            }
        }

        Ok::<(), SendError<GatewayClientEventMessage>>(())
    });

    // receive payload from worker process
    let mut resp = client
        .client_event_streaming(ReceiverStream::new(rx))
        .await
        .into_diagnostic()?
        .into_inner();
    while let Some(res) = resp.next().await
        && !terminator.has_changed().unwrap()
    {
        let Ok(response) = res else {
            continue;
        };

        match response.status() {
            GatewayClientEventResponseStatus::StatusHandled => {
                hartex_tracing::debug!("event handled");
            }
            GatewayClientEventResponseStatus::StatusRequestGuildMembers => {
                let guild_id = response.guild_id();

                hartex_tracing::debug!("guild members for guild {guild_id} requested");

                let command =
                    RequestGuildMembersBuilder::new(Id::new(u64::from_str(guild_id).unwrap()))
                        .query("", Some(1000));

                shard.lock().await.command(&command);
            }
        }
    }

    Ok(())
}
