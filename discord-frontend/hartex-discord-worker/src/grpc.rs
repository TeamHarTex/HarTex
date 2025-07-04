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

use hartex_discord_core::discord::cache::DefaultInMemoryCache;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::sync::mpsc;
use hartex_discord_grpc_protos::gateway::GatewayClientEventMessage;
use hartex_discord_grpc_protos::gateway::GatewayClientEventResponse;
use hartex_discord_grpc_protos::gateway::gateway_server::Gateway;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tonic::Response;
use tonic::Result;
use tonic::Streaming;
use tonic::async_trait;

pub struct GatewayWorkerServer {
    pub(crate) cache: DefaultInMemoryCache,
}

#[async_trait]
impl Gateway for GatewayWorkerServer {
    type ClientEventStreamingStream = ReceiverStream<Result<GatewayClientEventResponse>>;

    async fn client_event_streaming(
        &self,
        request: Request<Streaming<GatewayClientEventMessage>>,
    ) -> Result<Response<Self::ClientEventStreamingStream>> {
        let mut stream = request.into_inner();
        let (_, rx) = mpsc::channel(1000);

        tokio::spawn(async move { while let Some(_) = stream.next().await {} });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
