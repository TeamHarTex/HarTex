/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use color_eyre::Result;
use hartex_discord_actors::leader::ShardManager;
use hartex_discord_envconf::load_configuration;
use hartex_tracing::{self, eyre};
use kameo::actor::Spawn;
use mimalloc::MiMalloc;
use tokio::signal;
use tracing::subscriber;

mod shards;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

#[tokio::main]
pub async fn main() -> Result<()> {
    hartex_termios_utils::no_echoctl();
    eyre::initialize_eyre()?;
    subscriber::set_global_default(hartex_tracing::subscriber())?;

    tracing::trace!("loading environment variables...");
    let config = load_configuration()?;

    // todo: communicate with shard manager first

    tracing::info!("{}", hartex_version::version());
    tracing::info!("worker starting up...");

    let shards = shards::create(config.token().to_owned()).await?.collect::<Vec<_>>();
    let shard_manager_ref = ShardManager::spawn(shards);

    signal::ctrl_c().await?;
    shard_manager_ref.stop_gracefully().await?;
    shard_manager_ref.wait_for_shutdown_result().await.unwrap();

    Ok(())
}
