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

use hartex_discord_core::discord::gateway::Shard;
use hartex_kafka_utils::traits::ClientConfigUtils;
use hartex_kafka_utils::types::CompressionType;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::ClientConfig;

#[allow(clippy::unused_async)]
pub async fn listen(_: Vec<Shard>) -> hartex_discord_eyre::Result<()> {
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")?
        .split(';')
        .map(String::from)
        .collect::<Vec<_>>();
    let topic = env::var("KAFKA_LEADER_TOPIC_DISCORD_GATEWAY_COMMAND")?;

    let consumer = ClientConfig::new()
        .bootstrap_servers(bootstrap_servers.into_iter())
        .compression_type(CompressionType::Lz4)
        .group_id("com.github.teamhartex.hartex.inbound.gateway.payload.consumer")
        .create::<StreamConsumer>()?;

    consumer.subscribe(&[&topic])?;

    Ok(())
}
