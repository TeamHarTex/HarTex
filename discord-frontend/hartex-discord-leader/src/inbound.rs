/*
 * SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
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
use lapin::options::BasicPublishOptions;
use lapin::BasicProperties;
use lapin::Channel;

pub async fn handle(shards: impl Iterator<Item = &mut Shard>, amqp: Channel) {
    let mut stream = ShardMessageStream::new(shards);

    while let Some((shard, result)) = stream.next().await {
        match result {
            Ok(message) => {
                let Some(bytes) = (match message {
                    Message::Binary(bytes) => Some(bytes),
                    Message::Text(string) => Some(string.into_bytes()),
                    _ => None,
                }) else {
                    continue
                };

                log::trace!(
                    "[shard {shard_id}] received binary payload from gateway",
                    shard_id = shard.id().number()
                );

                if let Err(error) = amqp
                    .basic_publish(
                        "gateway",
                        &format!("SHARD {} PAYLOAD", shard.id().number()),
                        BasicPublishOptions::default(),
                        bytes.as_slice(),
                        BasicProperties::default(),
                    )
                    .await
                {
                    log::warn!(
                        "[shard {shard_id}] failed to publish payload to worker: {error}",
                        shard_id = shard.id().number()
                    );
                }
            }
            Err(error) => {
                if error.is_fatal() {
                    log::error!(
                        "[shard {shard_id}] FATAL ERROR WHEN RECEIVING GATEWAY MESSAGE: {error}; TERMINATING EVENT LOOP",
                        shard_id = shard.id().number()
                    );
                    break;
                }

                log::warn!(
                    "[shard {shard_id}] error when receiving gateway message: {error}",
                    shard_id = shard.id().number()
                );
            }
        }
    }
}
