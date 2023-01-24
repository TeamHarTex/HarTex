/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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
use std::time::Duration;

use futures_util::StreamExt;
use hartex_discord_core::discord::gateway::stream::ShardMessageStream;
use hartex_discord_core::discord::gateway::Message;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::log;
use hartex_discord_eyre::eyre::Report;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;

pub async fn handle(
    shards: impl Iterator<Item = &mut Shard>,
    producer: FutureProducer,
) -> hartex_discord_eyre::Result<()> {
    let mut stream = ShardMessageStream::new(shards);
    let topic = env::var("KAFKA_TOPIC_DISCORD_GATEWAY_PAYLOAD")?;

    while let Some((shard, result)) = stream.next().await {
        match result {
            Ok(message) => {
                let Some(bytes) = (match message {
                    // todo: handle close frame
                    Message::Close(_) => None,
                    Message::Text(string) => Some(string.into_bytes()),
                }) else {
                    continue
                };

                log::trace!(
                    "[shard {shard_id}] received binary payload from gateway",
                    shard_id = shard.id().number()
                );

                if let Err((error, _)) = producer
                    .send(
                        FutureRecord::to(&topic)
                            .key(&format!(
                                "INBOUND_GATEWAY_PAYLOAD_SHARD_{shard_id}",
                                shard_id = shard.id().number()
                            ))
                            .payload(&bytes),
                        Timeout::After(Duration::from_secs(0)),
                    )
                    .await
                {
                    println!("{:?}", Report::new(error));

                    continue;
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

    Ok(())
}
