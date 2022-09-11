/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use futures_util::{Stream, StreamExt};
use hartex_core::discord::gateway::Event;
use hartex_core::log;

pub async fn handle_inbound(
    cluster_id: usize,
    mut events: impl Stream<Item = (u64, Event)> + Send + Sync + Unpin + 'static,
) {
    while let Some((shard_id, event)) = events.next().await {
        match event {
            Event::GatewayHello(heartbeat_interval) => {
                log::trace!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_HELLO (heartbeat interval: {heartbeat_interval})");
            }
            Event::GatewayInvalidateSession(resumable) => {
                log::trace!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_INVALID_SESSION (resumable: {resumable})");
            }
            Event::Ready(_) => {
                log::info!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_READY");
            }
            Event::Resumed => {
                log::info!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_RESUMED");
            }
            Event::ShardConnected(_) => {
                log::info!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_CONNECTED");
            }
            Event::ShardConnecting(payload) => {
                log::info!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_CONNECTING (gateway uri: {})", payload.gateway);
            }
            Event::ShardDisconnected(payload) => {
                if let Some(code) = payload.code {
                    let reason = payload.reason.unwrap_or_default();
                    if !reason.is_empty() {
                        log::warn!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_DISCONNECTED (code: {code}, reason: {reason})",);
                    } else {
                        log::warn!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_DISCONNECTED (code: {code})");
                    }
                } else {
                    log::warn!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_DISCONNECTED");
                }
            }
            Event::ShardIdentifying(_) => {
                log::info!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_IDENTIFYING");
            }
            Event::ShardReconnecting(_) => {
                log::info!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_RECONNECTING");
            }
            Event::ShardResuming(payload) => {
                log::info!(
                    "[cluster {cluster_id} - shard {shard_id}] GATEWAY_RESUMING (sequence: {})",
                    payload.seq
                );
            }
            _ => {}
        }
    }
}
