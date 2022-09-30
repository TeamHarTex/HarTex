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

use futures_util::StreamExt;
use hartex_core::discord::gateway::stream::ShardEventStream;
use hartex_core::discord::gateway::{Event, Shard};
use hartex_core::discord::model::gateway::payload::incoming::{Hello, Ready};
use hartex_core::log;

pub async fn handle_inbound(cluster_id: usize, mut cluster: Vec<Shard>) {
    let mut stream = ShardEventStream::new(cluster.iter_mut());

    while let Some((shard, result)) = stream.next().await {
        match result {
            Ok(event) => match event {
                Event::GatewayHello(Hello { heartbeat_interval }) => {
                    log::trace!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_HELLO (heartbeat interval: {heartbeat_interval})", shard_id = shard.id().number());
                }
                Event::GatewayInvalidateSession(resumable) => {
                    log::trace!("[cluster {cluster_id} - shard {shard_id}] GATEWAY_INVALID_SESSION (resumable: {resumable})", shard_id = shard.id().number());
                }
                Event::Ready(_) => {
                    log::info!(
                        "[cluster {cluster_id} - shard {shard_id}] GATEWAY_READY",
                        shard_id = shard.id().number()
                    );
                }
                Event::Resumed => {
                    log::info!(
                        "[cluster {cluster_id} - shard {shard_id}] GATEWAY_RESUMED",
                        shard_id = shard.id().number()
                    );
                }
                _ => {}
            },
            Err(error) => {
                if error.is_fatal() {
                    break;
                }
            }
        }
    }
}
