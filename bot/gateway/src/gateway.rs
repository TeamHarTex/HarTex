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

use futures_util::StreamExt;
use protocol::buffers::gateway::{Command, GatewayCommand};
use tokio::{signal, task::JoinSet};
use twilight_gateway::{Config, Intents, Shard};
use twilight_http::Client;

use crate::{
    error::GatewayResult,
    nats::{GatewayCommandStream, conversion},
    shard::{ShardFuture, ShardHandle},
};

pub struct GatewayRunner {
    commands: GatewayCommandStream,
    futures: Vec<ShardFuture>,
    handles: HashMap<u32, ShardHandle>,
}

impl GatewayRunner {
    pub async fn new(token: String, nats_server: String) -> GatewayResult<Self> {
        let client = Client::new(token.clone());
        let connect_info = client.gateway().authed().await?.model().await?;

        let nats_client = async_nats::connect(nats_server).await?;
        let commands = GatewayCommandStream::new(nats_client).await?;

        // todo: use only necessary intents
        let shard_config = Config::new(token, Intents::all());
        let (handles, futures) = twilight_gateway::bucket(0, 1, connect_info.shards)
            .zip(iter::repeat_n(shard_config, connect_info.shards as usize))
            .map(|(id, config)| Shard::with_config(id, config))
            .map(|shard| {
                let shard_id = shard.id().number();
                let handle = ShardHandle::new(shard.sender());
                let runner = ShardFuture::new(shard);

                ((shard_id, handle), runner)
            })
            .unzip();

        Ok(GatewayRunner {
            commands,
            futures,
            handles,
        })
    }

    pub async fn run(self) -> GatewayResult<()> {
        let Self {
            mut commands,
            futures,
            handles,
        } = self;

        let mut tasks = JoinSet::new();
        futures.into_iter().for_each(|f| {
            tasks.spawn(f);
        });

        let ctrl_c = signal::ctrl_c();
        tokio::pin!(ctrl_c);

        loop {
            tokio::select! {
                Some(result) = commands.next() => match result {
                    Ok(command) => Self::dispatch_command(&handles, command)?,
                    Err(err) => {
                        tracing::warn!("failed to receive shard command: {err}");
                        continue;
                    },
                },
                option = tasks.join_next() => {
                    let Some(result) = option else {
                        tracing::info!("all shard tasks have completed, gateway runner stopping");
                        break;
                    };

                    match result {
                        Ok(Ok(())) => continue,
                        Ok(Err(gateway)) => todo!(),
                        Err(join) => todo!(),
                    }
                }
                _ = &mut ctrl_c => break,
            }
        }

        Ok(())
    }

    fn dispatch_command(
        handles: &HashMap<u32, ShardHandle>,
        gateway_command: GatewayCommand,
    ) -> GatewayResult<()> {
        let Some(handle) = handles.get(&gateway_command.shard_id) else {
            unreachable!("invalid shard id in command: {}", gateway_command.shard_id);
        };

        let Some(command) = gateway_command.command else {
            unreachable!("command must be one of the variants");
        };

        let cmd = match command {
            Command::RequestGuildMembers(request) => conversion::request_guild_members(request)?,
        };

        handle.send(cmd)
    }
}
