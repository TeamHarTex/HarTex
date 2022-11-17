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

use std::str;

use futures_util::StreamExt;
use hartex_discord_core::discord::model::gateway::event::GatewayEventDeserializer;
use hartex_discord_core::dotenvy;
use hartex_discord_core::log;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::signal;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties};
use serde::de::DeserializeSeed;
use serde_scan::scan;
use hartex_discord_eyre::eyre::Report;

use crate::error::{ConsumerError, ConsumerErrorKind};

mod entitycache;
mod error;
mod eventcallback;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_discord_eyre::Result<()> {
    hartex_discord_eyre::initialize()?;
    log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    let username = std::env::var("GATEWAY_RABBITMQ_USERNAME")?;
    let password = std::env::var("GATEWAY_RABBITMQ_PASSWORD")?;
    let host = std::env::var("RABBITMQ_HOST")?;
    let port = std::env::var("RABBITMQ_PORT")?;
    let uri = format!("amqp://{username}:{password}@{host}:{port}");
    let uri_log = format!("amqp://{username}:<redacted>@{host}:{port}");

    log::trace!("creating rabbitmq amqp connection (uri: {})", &uri_log);
    let amqp_connection = Connection::connect(&uri, ConnectionProperties::default()).await?;

    let channel_inbound = amqp_connection.create_channel().await?;
    let mut consumer = channel_inbound
        .basic_consume(
            "gateway.inbound",
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(result) = consumer.next().await {
        if let Ok(delivery) = result {
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("failed to ack");
            let value = delivery.routing_key.as_str();
            let scanned: u8 = scan!("SHARD {} PAYLOAD" <- value)?;

            let (gateway_deserializer, mut json_deserializer) = {
                let result = str::from_utf8(&delivery.data);
                if let Err(error) = result {
                    let report = Report::new(error);
                    println!("{report}");

                    continue;
                }

                let result = GatewayEventDeserializer::from_json(result.unwrap())
                .ok_or(ConsumerError {
                    kind: ConsumerErrorKind::InvalidGatewayPayload,
                    source: None,
                });

                if let Err(error) = result.clone() {
                    let report = Report::new(error);
                    println!("{report}");

                    continue;
                }

                let json_deserializer = serde_json::Deserializer::from_slice(&delivery.data);

                (result.unwrap(), json_deserializer)
            };

            let result = gateway_deserializer
                .clone()
                .deserialize(&mut json_deserializer);
            if let Err(error) = result {
                let report = Report::new(error);
                println!("{report}");

                continue;
            }

            log::trace!(
                "[shard {}] received {} event",
                scanned,
                gateway_deserializer.event_type_ref().unwrap_or("UNKNOWN")
            );
            // entitycache::update_entitycache(&event).await?;
            eventcallback::handle_event(result.unwrap())?;
        }
    }

    signal::ctrl_c().await?;
    log::warn!("ctrl-c signal received, shutting down");

    Ok(())
}
