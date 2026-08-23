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
mod command;
mod handle;

use std::iter;

use tokio::{signal, sync::mpsc::{self, Receiver}, task::JoinSet};
use twilight_gateway::{Config, Intents, Shard};
use twilight_http::Client;

pub use crate::gateway::handle::GatewayHandle;
use crate::{error::GatewayResult, gateway::command::GatewayCommand, shard};

pub struct Gateway {
    shards: Vec<Shard>,
    rx: Receiver<GatewayCommand>,
}

impl Gateway {
    pub async fn new(token: String) -> GatewayResult<(Self, GatewayHandle)> {
        let client = Client::new(token.clone());
        let connect_info = client.gateway().authed().await?.model().await?;

        // todo: use only necessary intents
        let shard_config = Config::new(token, Intents::all());
        let shards = twilight_gateway::bucket(0, 1, connect_info.shards)
            .zip(iter::repeat_n(shard_config, connect_info.shards as usize))
            .map(|(id, config)| Shard::with_config(id, config))
            .collect();

        let (tx, rx) = mpsc::channel(1024);
        let handle = GatewayHandle(tx);

        Ok((Gateway { shards, rx }, handle))
    }

    pub async fn run(mut self) {
        let mut tasks = JoinSet::new();
        self.shards.into_iter().for_each(|shard| {
            tasks.spawn(shard::runner(shard));
        });

        loop {
            tokio::select! {
                _ = self.rx.recv() => {},
                _ = tasks.join_next() => {},
                _ = signal::ctrl_c() => break,
            }
        }
    }
}
