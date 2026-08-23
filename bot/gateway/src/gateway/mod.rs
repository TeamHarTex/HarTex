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

use std::{collections::HashMap, iter};

use tokio::{
    signal,
    sync::mpsc::{self, Receiver},
    task::JoinSet,
};
use twilight_gateway::{Config, Intents, Shard};
use twilight_http::Client;
use twilight_model::gateway::ShardId;

pub use crate::gateway::handle::GatewayHandle;
use crate::{
    command::GatewayCommand,
    error::GatewayResult,
    shard::{ShardFuture, ShardHandle},
};

mod handle;

pub struct GatewayRunner {
    futures: Vec<ShardFuture>,
    handles: HashMap<ShardId, ShardHandle>,
    rx: Receiver<GatewayCommand>,
}

impl GatewayRunner {
    pub async fn new(token: String) -> GatewayResult<(Self, GatewayHandle)> {
        let client = Client::new(token.clone());
        let connect_info = client.gateway().authed().await?.model().await?;

        // todo: use only necessary intents
        let shard_config = Config::new(token, Intents::all());
        let (handles, futures) = twilight_gateway::bucket(0, 1, connect_info.shards)
            .zip(iter::repeat_n(shard_config, connect_info.shards as usize))
            .map(|(id, config)| Shard::with_config(id, config))
            .map(|shard| {
                let shard_id = shard.id();
                let handle = ShardHandle::new(shard.sender());
                let runner = ShardFuture::new(shard);

                ((shard_id, handle), runner)
            })
            .unzip();

        let (tx, rx) = mpsc::channel(1024);
        let handle = GatewayHandle::new(tx);

        Ok((
            GatewayRunner {
                futures,
                handles,
                rx,
            },
            handle,
        ))
    }

    pub async fn run(self) -> GatewayResult<()> {
        let Self {
            futures,
            handles,
            mut rx,
        } = self;

        let mut tasks = JoinSet::new();
        futures.into_iter().for_each(|f| {
            tasks.spawn(f);
        });

        loop {
            tokio::select! {
                Some(command) = rx.recv() => Self::dispatch_command(&handles, command)?,
                _ = tasks.join_next() => {},
                _ = signal::ctrl_c() => break,
            }
        }

        Ok(())
    }

    fn dispatch_command(handles: &HashMap<ShardId, ShardHandle>, command: GatewayCommand) -> GatewayResult<()> {
        let Some(handle) = handles.get(&command.shard()) else {
            unreachable!("shard handle not found, this should never happen");
        };

        handle.send(command.into_command())?;
        Ok(())
    }
}
