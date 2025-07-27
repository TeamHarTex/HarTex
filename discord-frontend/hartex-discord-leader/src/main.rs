/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::sync::Arc;

use hartex_discord_core::{
    discord::gateway::CloseFrame,
    dotenvy, tokio,
    tokio::{
        signal,
        sync::{Mutex, watch},
        task::JoinSet,
    },
};
use hartex_discord_grpc_protos::gateway::gateway_client::GatewayClient;
use miette::IntoDiagnostic;
use mimalloc::MiMalloc;

mod grpc;
mod queue;
mod shards;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Entry point.
#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> miette::Result<()> {
    tracing::subscriber::set_global_default(hartex_tracing::subscriber()).unwrap();

    hartex_tracing::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    hartex_tracing::trace!("building clusters");
    let queue = queue::obtain()?;
    let shards = shards::obtain(queue).await?;

    let (tx, rx) = watch::channel(false);

    let client = GatewayClient::connect("http://127.0.0.1:6553").await.unwrap();

    hartex_tracing::trace!("launching {shards.len()} shard(s)");
    let mut set = JoinSet::new();
    for shard in shards {
        let mut rx = rx.clone();
        let client_cloned = client.clone();

        let mutex_shard = Arc::new(Mutex::new(shard));
        let shard_cloned = Arc::clone(&mutex_shard);

        set.spawn(async move {
            tokio::select! {
                _ = grpc::handle(shard_cloned, client_cloned) => {},
                _ = rx.changed() => {
                    mutex_shard.lock().await.close(CloseFrame::NORMAL);
                }
            }
        });
    }

    signal::ctrl_c().await.into_diagnostic()?;

    hartex_tracing::warn!("ctrl-c signal received, shutting down");

    tx.send(true).into_diagnostic()?;

    // wait for all tasks to complete
    while set.join_next().await.is_some() {}

    Ok(())
}
