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

use std::collections::HashMap;

use hartex_core::dotenv;
use hartex_core::log;
use hartex_core::tokio;
use hartex_core::tokio::signal;
use lapin::options::{ExchangeDeclareOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties, ExchangeKind};

mod clusters;
mod error;
mod queue;
mod sessions;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;
    log::initialize();

    log::trace!("loading environment variables");
    dotenv::dotenv()?;

    let username = std::env::var("GATEWAY_RABBITMQ_USERNAME")?;
    let password = std::env::var("GATEWAY_RABBITMQ_PASSWORD")?;
    let host = std::env::var("RABBITMQ_HOST")?;
    let port = std::env::var("RABBITMQ_PORT")?;
    let uri = format!("amqp://{}:{}@{}:{}", username, password, host, port);
    let uri_log = format!("amqp://{}:<redacted>@{}:{}", username, host, port);

    log::trace!("creating rabbitmq amqp connection (uri: {})", &uri_log);
    let amqp_connection = Connection::connect(&uri, ConnectionProperties::default()).await?;

    let channel_recv = amqp_connection.create_channel().await?;
    let channel_send = amqp_connection.create_channel().await?;

    log::trace!("declaring amqp exchange");
    channel_recv
        .exchange_declare(
            "gateway",
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                passive: false,
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    log::trace!("declaring amqp send queue");
    channel_send
        .queue_declare(
            "gateway.send",
            QueueDeclareOptions {
                passive: false,
                durable: true,
                exclusive: false,
                auto_delete: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    log::trace!("building clusters");
    let shards = std::env::var("NUM_SHARDS")?.parse::<u64>()?;
    let resume_sessions = sessions::get_sessions().await?;
    let queue = queue::get_queue()?;
    let (clusters, events) = clusters::get_clusters(shards, queue, resume_sessions.clone()).await?;

    log::trace!(
        "launching {} cluster(s) with {shards} shard(s); resuming {} session(s)",
        clusters.len(),
        resume_sessions.len(),
    );
    for (cluster, _) in clusters.clone().into_iter().zip(events.into_iter()) {
        let cluster_clone = cluster.clone();
        tokio::spawn(async move {
            cluster_clone.up().await;
        });
    }

    signal::ctrl_c().await?;

    log::trace!("shutting down, storing resumable sessions");
    let mut sessions = HashMap::new();
    for cluster in clusters {
        for (key, value) in cluster.down_resumable() {
            sessions.insert(key, value);
        }
    }

    sessions::set_sessions(sessions).await?;

    Ok(())
}
