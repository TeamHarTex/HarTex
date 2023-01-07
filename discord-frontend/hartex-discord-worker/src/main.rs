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

#![deny(clippy::pedantic)]
#![deny(warnings)]

use std::env;
use std::str;

use futures_util::StreamExt;
use hartex_discord_core::discord::model::gateway::event::GatewayEventDeserializer;
use hartex_discord_core::dotenvy;
use hartex_discord_core::log;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::signal;
use hartex_discord_eyre::eyre::Report;
use hartex_kafka_utils::traits::ClientConfigUtils;
use hartex_kafka_utils::types::CompressionType;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use serde::de::DeserializeSeed;
use serde_scan::scan;

use crate::error::ConsumerError;
use crate::error::ConsumerErrorKind;

mod entitycache;
mod error;
mod eventcallback;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_discord_eyre::Result<()> {
    hartex_discord_eyre::initialize()?;
    log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")?
        .split(";")
        .map(String::from)
        .collect::<Vec<_>>();
    let topic = env::var("KAFKA_LEADER_TOPIC_DISCORD_GATEWAY_PAYLOAD")?;

    let consumer = ClientConfig::new()
        .bootstrap_servers(bootstrap_servers.into_iter())
        .compression_type(CompressionType::Lz4)
        .group_id("com.github.teamhartex.hartex.inbound.gateway.payload.consumer")
        .create::<StreamConsumer>()?;

    consumer.subscribe(&[&topic])?;

    while let Some(result) = consumer.stream().next().await {
        let Ok(message) = result else {
            let error = result.unwrap_err();
            println!("{:?}", Report::new(error));

            continue;
        };

        let bytes = message.payload().unwrap();

        let (gateway_deserializer, mut json_deserializer) = {
            let result = str::from_utf8(bytes);
            if let Err(error) = result {
                println!("{:?}", Report::new(error));

                continue;
            }

            let result =
                GatewayEventDeserializer::from_json(result.unwrap()).ok_or(ConsumerError {
                    kind: ConsumerErrorKind::InvalidGatewayPayload,
                });

            if let Err(error) = result {
                println!("{:?}", Report::new(error));

                continue;
            }

            let json_deserializer = serde_json::Deserializer::from_slice(bytes);

            (result.unwrap(), json_deserializer)
        };

        let key_bytes = message.key().unwrap();
        let result = str::from_utf8(key_bytes);
        if let Err(error) = result {
            println!("{:?}", Report::new(error));

            continue;
        }

        let key = result.unwrap();
        let scanned: u8 = scan!("INBOUND_GATEWAY_PAYLOAD_SHARD_{}" <- key)?;

        log::trace!(
            "[shard {scanned}] received {} event; attempting to deserialize",
            gateway_deserializer.event_type().unwrap_or("UNKNOWN")
        );
        let result = gateway_deserializer.deserialize(&mut json_deserializer);
        if let Err(error) = result {
            println!("{:?}", Report::new(error));

            continue;
        }

        let event = result.unwrap();

        let (Ok(update_result), Ok(event_result)) = tokio::join!(
                tokio::spawn(entitycache::update(event.clone())),
                tokio::spawn(eventcallback::invoke(event))
            ) else {
            log::trace!("failed to join futures; skipping event");
            continue;
        };

        if let Err(report) = update_result {
            println!("{report:?}");
        }

        if let Err(report) = event_result {
            println!("{report:?}");
        }
    }

    signal::ctrl_c().await?;
    log::warn!("ctrl-c signal received, shutting down");

    Ok(())
}
