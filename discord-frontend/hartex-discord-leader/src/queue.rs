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

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::log;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::sync::mpsc::UnboundedReceiver;
use hartex_discord_core::tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use hartex_discord_core::tokio::sync::oneshot::{self, Sender};
use hartex_discord_core::tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct LocalQueue(UnboundedSender<Sender<()>>);

impl LocalQueue {
    pub fn new(duration: Duration) -> Self {
        let (tx, rx) = unbounded_channel();
        tokio::spawn(wait_for_while(rx, duration));

        Self(tx)
    }
}

impl Queue for LocalQueue {
    fn request(&'_ self, [_, _]: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let (tx, rx) = oneshot::channel::<()>();

            if let Err(error) = self.0.clone().send(tx) {
                log::warn!("skipping, send failed: {:?}", error);
                return;
            }

            let _ = rx.await;
        })
    }
}

#[derive(Debug)]
pub struct LargeBotQueue(Vec<UnboundedSender<Sender<()>>>);

impl LargeBotQueue {
    pub fn new(buckets: usize, duration: Duration) -> Self {
        let mut queues = Vec::with_capacity(buckets);
        for _ in 0..buckets {
            let (tx, rx) = unbounded_channel();
            tokio::spawn(wait_for_while(rx, duration));
            queues.push(tx)
        }

        Self(queues)
    }
}

impl Queue for LargeBotQueue {
    fn request(&'_ self, shard_id: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let bucket = (shard_id[0] % (self.0.len() as u64)) as usize;
        let (tx, rx) = oneshot::channel();

        Box::pin(async move {
            if let Err(error) = self.0[bucket].clone().send(tx) {
                log::warn!("skipping, send failed: {:?}", error);
                return;
            }

            let _ = rx.await;
        })
    }
}

async fn wait_for_while(mut rx: UnboundedReceiver<Sender<()>>, duration: Duration) {
    while let Some(tx) = rx.recv().await {
        if let Err(error) = tx.send(()) {
            log::warn!("skipping, send failed: {:?}", error);
        }

        sleep(duration).await;
    }
}

pub fn get_queue() -> hartex_discord_eyre::Result<Arc<dyn Queue>> {
    let concurrency = std::env::var("SHARD_CONCURRENCY")?.parse::<usize>()?;
    let wait =
        Duration::from_secs(std::env::var("SHARD_CONCURRENCY_WAIT_SECONDS")?.parse::<u64>()?);

    if concurrency == 1 {
        Ok(Arc::new(LocalQueue::new(wait)))
    } else {
        Ok(Arc::new(LargeBotQueue::new(concurrency, wait)))
    }
}
