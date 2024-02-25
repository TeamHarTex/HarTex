/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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
use std::str;
use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt as FutureStreamExt;
use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::discord::gateway::Message as GatewayMessage;
use hartex_discord_core::discord::gateway::MessageSender;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::discord::model::gateway::payload::outgoing::RequestGuildMembers;
use hartex_discord_core::tokio;
use hartex_log::log;
use miette::IntoDiagnostic;
use rdkafka::consumer::StreamConsumer;
use rdkafka::error::KafkaError;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use rdkafka::Message;
use serde_scan::scan;

/// Handle inbound AND outbound messages for a given shard.
pub async fn handle<'a, Q>(
    shard: &mut Shard<Q>,
    producer: FutureProducer,
    consumer: Arc<StreamConsumer>,
) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    let shard_id = shard.id().number();
    let sender = shard.sender();
    tokio::select! {
        _ = inbound(shard, producer) => {},
        _ = outbound((shard_id, sender), consumer) => {}
    }

    Ok(())
}

#[allow(clippy::match_wildcard_for_single_variants)]
async fn inbound<Q>(shard: &mut Shard<Q>, producer: FutureProducer) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    let topic = env::var("KAFKA_TOPIC_INBOUND_DISCORD_GATEWAY_PAYLOAD").into_diagnostic()?;

    while let Some(result) = shard.next().await {
        match result {
            Ok(message) => {
                let Some(bytes) = (match message {
                    // todo: handle close frame
                    GatewayMessage::Text(string) => Some(string.into_bytes()),
                    _ => None,
                }) else {
                    continue;
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
                    println!("{:?}", Err::<(), KafkaError>(error).into_diagnostic());

                    continue;
                }
            }
            Err(error) => {
                log::warn!(
                    "[shard {shard_id}] error when receiving gateway message: {error}",
                    shard_id = shard.id().number()
                );
            }
        }
    }

    Ok(())
}

async fn outbound(
    (shard_id, sender): (u32, MessageSender),
    consumer: Arc<StreamConsumer>,
) -> miette::Result<()> {
    while let Some(result) = consumer.stream().next().await {
        let Ok(message) = result else {
            let error = result.unwrap_err();
            println!("{:?}", Err::<(), KafkaError>(error).into_diagnostic());

            continue;
        };

        let key = str::from_utf8(message.key().unwrap()).unwrap();

        if key.contains("REQUEST_GUILD_MEMBERS") {
            let bytes = message.payload().unwrap();

            let command = serde_json::from_slice::<RequestGuildMembers>(bytes).into_diagnostic()?;
            let scanned: u32 =
                scan!("OUTBOUND_REQUEST_GUILD_MEMBERS_{}" <- key).into_diagnostic()?;

            if shard_id != scanned {
                continue;
            }

            sender.command(&command).into_diagnostic()?;
        }
    }

    Ok(())
}
