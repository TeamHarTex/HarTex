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

use std::{
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use bytes::{Bytes, BytesMut};
use hartex_discord_core::{
    discord::{cache::DefaultInMemoryCache, model::gateway::event::GatewayEventDeserializer},
    tokio,
    tokio::sync::{mpsc, mpsc::Sender},
};
use hartex_discord_grpc_protos::gateway::{
    GatewayClientEventMessage, GatewayClientEventResponse, gateway_server::Gateway,
};
use parking_lot::Mutex;
use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Result, Status, Streaming, async_trait};

/// A gateway worker server service.
pub struct GatewayWorkerServer {
    cache: Arc<DefaultInMemoryCache>,
    payloads: GatewayPayloads,
}

impl GatewayWorkerServer {
    pub fn new(cache: DefaultInMemoryCache) -> Self {
        Self {
            cache: Arc::new(cache),
            payloads: GatewayPayloads(Arc::new(Mutex::new(BTreeMap::new()))),
        }
    }
}

#[derive(Clone)]
struct GatewayPayloads(Arc<Mutex<BTreeMap<(u64, u64), GatewayPayloadChunked>>>);

impl GatewayPayloads {
    pub fn consistent_totals(&self, event_seq: u64, received_total: u32, shard_id: u64) -> bool {
        let mut payloads = self.0.lock();
        let entry = payloads.entry((event_seq, shard_id));
        let Entry::Occupied(entry) = entry else {
            // this is a new event sequence, total always assumed to be consistent
            return true;
        };

        entry.get().total == received_total
    }

    pub fn try_insert_payload_with_completeness_check(
        &self,
        event_seq: u64,
        nth: u32,
        total: u32,
        shard_id: u64,
        data: Bytes,
        tx: &Sender<(u64, BTreeMap<u32, Bytes>)>,
    ) -> bool {
        let mut payloads = self.0.lock();
        let chunked = payloads
            .entry((event_seq, shard_id))
            .or_insert(GatewayPayloadChunked {
                total,
                received: 0,
                chunks: BTreeMap::new(),
            });

        if chunked.chunks.try_insert(nth, data).is_err() {
            return false; // duplicate payload chunk
        }
        chunked.received += 1;

        if chunked.received == chunked.total {
            let payload = payloads.remove(&(event_seq, shard_id)).unwrap();
            tx.try_send((shard_id, payload.chunks)).unwrap();
        }

        true
    }
}

#[derive(Clone)]
struct GatewayPayloadChunked {
    total: u32,
    received: u32,
    chunks: BTreeMap<u32, Bytes>,
}

#[async_trait]
impl Gateway for GatewayWorkerServer {
    type ClientEventStreamingStream = ReceiverStream<Result<GatewayClientEventResponse>>;

    async fn client_event_streaming(
        &self,
        request: Request<Streaming<GatewayClientEventMessage>>,
    ) -> Result<Response<Self::ClientEventStreamingStream>> {
        let mut stream = request.into_inner();
        let (response_tx, response_rx) = mpsc::channel(32);
        let (internal_tx, mut internal_rx) = mpsc::channel(32);

        let payloads = self.payloads.clone();

        tokio::spawn(async move {
            while let Some(result) = stream.next().await {
                let Ok(message) = result else {
                    response_tx
                        .send(Err(Status::aborted(
                            "failed to retrieve message from payload",
                        )))
                        .await
                        .unwrap();

                    continue;
                };

                if !payloads.consistent_totals(
                    message.event_seq,
                    message.total_chunks,
                    message.shard_id,
                ) {
                    response_tx
                        .send(Err(Status::invalid_argument(
                            "inconsistent total_chunks for same event_seq",
                        )))
                        .await
                        .unwrap();

                    continue;
                }

                if !payloads.try_insert_payload_with_completeness_check(
                    message.event_seq,
                    message.nth_chunk,
                    message.total_chunks,
                    message.shard_id,
                    message.chunk_data,
                    &internal_tx,
                ) {
                    response_tx
                        .send(Err(Status::invalid_argument(format!(
                            "duplicate chunk {} in payload",
                            message.nth_chunk
                        ))))
                        .await
                        .unwrap();
                }
            }
        });

        let cache = self.cache.clone();

        tokio::spawn(async move {
            while let Some((shard_id, chunks)) = internal_rx.recv().await {
                let mut buffer = BytesMut::new();
                for chunk in chunks.values() {
                    buffer.extend_from_slice(chunk);
                }
                let done = buffer.freeze();

                let string = str::from_utf8(&done).unwrap();
                let deserializer = GatewayEventDeserializer::from_json(string).unwrap();
                let mut json = Deserializer::from_slice(&done);

                let event = deserializer.deserialize(&mut json).unwrap();
                crate::eventcallback::invoke(event, shard_id, cache.as_ref())
                    .await
                    .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(response_rx)))
    }
}
