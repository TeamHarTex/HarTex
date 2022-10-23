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
use hartex_discord_core::discord::gateway::message::Message;
use hartex_discord_core::discord::gateway::stream::ShardMessageStream;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::log;

pub async fn handle_inbound(cluster_id: usize, mut cluster: Vec<Shard>) {
    let mut stream = ShardMessageStream::new(cluster.iter_mut());

    while let Some((shard, result)) = stream.next().await {
        match result {
            Ok(message) => match message {
                msg => log::trace!("[cluster {cluster_id} - shard {shard_id}] {msg:?}", shard_id = shard.id().number())
            },
            Err(error) => {
                if error.is_fatal() {
                    log::error!(
                        "[cluster {cluster_id} - shard {shard_id}] FATAL ERROR WHEN RECEIVING GATEWAY MESSAGE: {error}; TERMINATING EVENT LOOP",
                        shard_id = shard.id().number()
                    );
                    break;
                }

                log::warn!(
                    "[cluster {cluster_id} - shard {shard_id}] error when receiving gateway message: {error}",
                    shard_id = shard.id().number()
                )
            }
        }
    }
}
