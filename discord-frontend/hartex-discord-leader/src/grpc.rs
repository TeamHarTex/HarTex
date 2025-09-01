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

use std::sync::Arc;

use bytes::Bytes;
use futures_util::StreamExt as FutureStreamExt;
use hartex_discord_core::{
    discord::gateway::{Message as GatewayMessage, Session, Shard, queue::Queue},
    tokio,
    tokio::sync::{Mutex, mpsc, mpsc::error::SendError},
};
use hartex_discord_grpc_protos::gateway::{
    GatewayClientEventMessage, gateway_client::GatewayClient,
    GatewayClientEventResponseStatus,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;

const CHUNK_SIZE: usize = 1024 * 1024;

/// Handle inbound AND outbound messages for a given shard.
pub async fn handle<Q>(
    shard: Arc<Mutex<Shard<Q>>>,
    client: GatewayClient<Channel>,
) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    tokio::select! {
        _ = inbound(shard.clone(), client) => {},
        _ = outbound(shard) => {}
    }

    Ok(())
}

/// Handle inbound traffic.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::match_wildcard_for_single_variants)]
async fn inbound<Q>(
    shard: Arc<Mutex<Shard<Q>>>,
    mut client: GatewayClient<Channel>,
) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    let (tx, rx) = mpsc::channel(1000);

    let cloned = Arc::clone(&shard);
    tokio::spawn(async move {
        let mut shard = cloned.lock().await;
        while let Some(result) = shard.next().await {
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

    // send payload to worker process
    let mut resp = client
        .client_event_streaming(ReceiverStream::new(rx))
        .await?
        .into_inner();
    while let Some(res) = resp.next().await {
        let Ok(response) = res else {
            continue;
        };

        match response.status {
            GatewayClientEventResponseStatus::StatusHandled => continue,
            GatewayClientEventResponseStatus::StatusRequestGuildMembers => todo!(),
        }
    }

    Ok(())
}

/// Handle outbound traffic.
async fn outbound<Q>(_: Arc<Mutex<Shard<Q>>>) -> miette::Result<()> {
    // while let Some(result) = consumer.stream().next().await {
    //     let Ok(message) = result else {
    //         let error = result.unwrap_err();
    //         println!("{:?}", Err::<(), KafkaError>(error).into_diagnostic());
    //
    //         continue;
    //     };
    //
    //     let key = str::from_utf8(message.key().unwrap()).unwrap();
    //
    //     if key.contains("REQUEST_GUILD_MEMBERS") {
    //         let bytes = message.payload().unwrap();
    //
    //         let command = serde_json::from_slice::<RequestGuildMembers>(bytes).into_diagnostic()?;
    //         let scanned: u32 =
    //             scan!("OUTBOUND_REQUEST_GUILD_MEMBERS_{}" <- key).into_diagnostic()?;
    //
    //         if shard_id != scanned {
    //             continue;
    //         }
    //
    //         sender.command(&command).into_diagnostic()?;
    //     }
    // }

    Ok(())
}
