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
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
use std::time::Duration;

use futures_util::future;
use hartex_discord_core::discord::gateway::CloseFrame;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::dotenvy;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::signal;
use hartex_discord_core::tokio::sync::watch;
use hartex_discord_core::tokio::time;
use hartex_kafka_utils::traits::ClientConfigUtils;
use hartex_kafka_utils::types::CompressionType;
use hartex_log::log;
use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;

mod inbound;
mod queue;
mod shards;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;
    hartex_log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")?
        .split(';')
        .map(String::from)
        .collect::<Vec<_>>();

    let producer = ClientConfig::new()
        .bootstrap_servers(bootstrap_servers.into_iter())
        .compression_type(CompressionType::Lz4)
        .delivery_timeout_ms(30000)
        .create::<FutureProducer>()?;

    log::trace!("building clusters");
    let num_shards = env::var("NUM_SHARDS")?.parse::<u64>()?;
    let queue = queue::obtain()?;
    let mut shards = shards::obtain(num_shards, &queue)?;

    let (tx, rx) = watch::channel(false);

    log::trace!("launching {num_shards} shard(s)");
    let mut rx = rx.clone();

    tokio::spawn(async move {
        tokio::select! {
            _ = inbound::handle(shards.iter_mut(), producer) => {},
            _ = rx.changed() => {
                future::join_all(shards.iter_mut().map(|shard: &mut Shard|shard.close(CloseFrame::RESUME))).await;
            },
        }
    });

    signal::ctrl_c().await?;

    log::warn!("ctrl-c signal received, shutting down");

    tx.send(true)?;
    time::sleep(Duration::from_secs(5)).await;

    Ok(())
}
