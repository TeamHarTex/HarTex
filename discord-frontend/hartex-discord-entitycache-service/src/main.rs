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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
use std::io::Error;
use std::io::ErrorKind;
use std::str::Utf8Error;
use std::str;

use futures_util::StreamExt;
use hartex_discord_core::discord::model::gateway::event::GatewayEventDeserializer;
use hartex_discord_core::dotenvy;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::signal;
use hartex_kafka_utils::traits::ClientConfigUtils;
use hartex_log::log;
use miette::IntoDiagnostic;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;
use rdkafka::Message;
use serde_scan::scan;

mod entitycache;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")
        .into_diagnostic()?
        .split(';')
        .map(String::from)
        .collect::<Vec<_>>();
    let topic = env::var("KAFKA_TOPIC_INBOUND_DISCORD_GATEWAY_PAYLOAD").into_diagnostic()?;

    let consumer = ClientConfig::new()
        .bootstrap_servers(bootstrap_servers.into_iter())
        .group_id("com.github.teamhartex.hartex.inbound.gateway.payload.consumer")
        .create::<StreamConsumer>()
        .into_diagnostic()?;

    consumer.subscribe(&[&topic]).into_diagnostic()?;

    while let Some(result) = consumer.stream().next().await {
        let Ok(message) = result else {
            let error = result.unwrap_err();
            println!("{:?}", Err::<(), KafkaError>(error).into_diagnostic());

            continue;
        };

        let bytes = message.payload().unwrap();

        let (gateway_deserializer, mut json_deserializer) = {
            let result = str::from_utf8(bytes);
            if let Err(error) = result {
                println!("{:?}", Err::<(), Utf8Error>(error).into_diagnostic());

                continue;
            }

            let result = GatewayEventDeserializer::from_json(result.unwrap())
                .ok_or(Error::new(ErrorKind::Other, ""))
                .into_diagnostic()?;

            let json_deserializer = serde_json::Deserializer::from_slice(bytes);

            (result, json_deserializer)
        };

        let key_bytes = message.key().unwrap();
        let result = str::from_utf8(key_bytes);
        if let Err(error) = result {
            println!("{:?}", Err::<(), Utf8Error>(error).into_diagnostic());

            continue;
        }

        let key = result.unwrap();
        let scanned: u8 = scan!("INBOUND_GATEWAY_PAYLOAD_SHARD_{}" <- key).into_diagnostic()?;

        log::trace!(
            "[shard {scanned}] received {} event; attempting to deserialize",
            gateway_deserializer.event_type().unwrap_or("UNKNOWN")
        );
        let result = gateway_deserializer.deserialize(&mut json_deserializer);
        if let Err(error) = result {
            println!(
                "{:?}",
                Err::<(), serde_json::Error>(error).into_diagnostic()
            );

            continue;
        }

        let event = result.unwrap();

        entitycache::update(event).await?;
    }

    signal::ctrl_c().await.into_diagnostic()?;
    log::warn!("ctrl-c signal received, shutting down");

    Ok(())
}
