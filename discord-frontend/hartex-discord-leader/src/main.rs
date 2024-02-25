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

//! # Leader Process
//!
//! The leader process is the process that connects to the Discord API, receives events and
//! forwards to the workers.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
use std::sync::Arc;
use std::time::Duration;

use hartex_discord_core::discord::gateway::CloseFrame;
use hartex_discord_core::dotenvy;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::signal;
use hartex_discord_core::tokio::sync::watch;
use hartex_discord_core::tokio::time;
use hartex_discord_utils::CLIENT;
use hartex_discord_utils::TOKEN;
use hartex_kafka_utils::traits::ClientConfigUtils;
use hartex_kafka_utils::types::CompressionType;
use hartex_log::log;
use miette::IntoDiagnostic;
use once_cell::sync::Lazy;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;

mod kafka;
mod queue;
mod shards;

/// Entry point.
#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    Lazy::force(&CLIENT);
    Lazy::force(&TOKEN);

    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")
        .into_diagnostic()?
        .split(';')
        .map(String::from)
        .collect::<Vec<_>>();
    let topic = env::var("KAFKA_TOPIC_OUTBOUND_COMMUNICATION").into_diagnostic()?;

    let producer = ClientConfig::new()
        .bootstrap_servers(bootstrap_servers.clone().into_iter())
        .compression_type(CompressionType::Lz4)
        .delivery_timeout_ms(30000)
        .create::<FutureProducer>()
        .into_diagnostic()?;
    let consumer = ClientConfig::new()
        .bootstrap_servers(bootstrap_servers.into_iter())
        .group_id("com.github.teamhartex.hartex.inbound.gateway.command.consumer")
        .create::<StreamConsumer>()
        .into_diagnostic()?;

    consumer.subscribe(&[&topic]).into_diagnostic()?;

    log::trace!("building clusters");
    let queue = queue::obtain()?;
    let shards = Arc::new(shards::obtain(queue).await?);

    let (tx, rx) = watch::channel(false);

    log::trace!("launching {} shard(s)", shards.len());
    let mut rx = rx.clone();

    tokio::spawn(async move {
        tokio::select! {
            _ = kafka::handle(shards.clone(), producer.clone(), consumer) => {},
            _ = rx.changed() => {
                let _ = shards.iter().map(|shard| shard.close(CloseFrame::RESUME));
            },
        }
    });

    signal::ctrl_c().await.into_diagnostic()?;

    log::warn!("ctrl-c signal received, shutting down");

    tx.send(true).into_diagnostic()?;
    time::sleep(Duration::from_secs(5)).await;

    Ok(())
}
