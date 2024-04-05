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

use std::time::Duration;

use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::tokio;
use hartex_discord_core::tokio::sync::mpsc::unbounded_channel;
use hartex_discord_core::tokio::sync::mpsc::UnboundedReceiver;
use hartex_discord_core::tokio::sync::mpsc::UnboundedSender;
use hartex_discord_core::tokio::sync::oneshot;
use hartex_discord_core::tokio::sync::oneshot::Receiver;
use hartex_discord_core::tokio::sync::oneshot::Sender;
use hartex_discord_core::tokio::time::sleep;
use hartex_log::log;
use miette::IntoDiagnostic;

/// A discriminated union of supported bot queues.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub enum BotQueue {
    /// A local queue.
    Local(LocalQueue),
    /// A large bot queue.
    LargeBot(LargeBotQueue),
}

impl Queue for BotQueue {
    fn enqueue(&self, id: u32) -> Receiver<()> {
        match self {
            Self::Local(local) => local.enqueue(id),
            Self::LargeBot(large_bot) => large_bot.enqueue(id),
        }
    }
}

/// A local queue.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct LocalQueue(UnboundedSender<Sender<()>>);

impl LocalQueue {
    /// Create a new local queue.
    pub fn new(duration: Duration) -> Self {
        let (tx, rx) = unbounded_channel();
        tokio::spawn(wait_for_while(rx, duration));

        Self(tx)
    }
}

impl Queue for LocalQueue {
    #[allow(unused_must_use)]
    fn enqueue(&'_ self, _: u32) -> Receiver<()> {
        let (tx, rx) = oneshot::channel::<()>();

        if let Err(error) = self.0.clone().send(tx) {
            log::warn!("skipping, send failed: {:?}", error);
        }

        rx
    }
}

/// A queue for large bots.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct LargeBotQueue(Vec<UnboundedSender<Sender<()>>>);

impl LargeBotQueue {
    /// Create a large bot queue.
    pub fn new(buckets: usize, duration: Duration) -> Self {
        let mut queues = Vec::with_capacity(buckets);
        for _ in 0..buckets {
            let (tx, rx) = unbounded_channel();
            tokio::spawn(wait_for_while(rx, duration));
            queues.push(tx);
        }

        Self(queues)
    }
}

impl Queue for LargeBotQueue {
    #[allow(unused_must_use)]
    fn enqueue(&'_ self, shard_id: u32) -> Receiver<()> {
        #[allow(clippy::cast_possible_truncation)]
        let bucket = (shard_id % (self.0.len() as u32)) as usize;
        let (tx, rx) = oneshot::channel();
        if let Err(error) = self.0[bucket].clone().send(tx) {
            log::warn!("skipping, send failed: {:?}", error);
        }

        rx
    }
}

/// Waits for a while until an unbounded receiver receives something.
async fn wait_for_while(mut rx: UnboundedReceiver<Sender<()>>, duration: Duration) {
    while let Some(tx) = rx.recv().await {
        if let Err(error) = tx.send(()) {
            log::warn!("skipping, send failed: {:?}", error);
        }

        sleep(duration).await;
    }
}

/// Obtain a queue to use for the startup of the bot.
pub fn obtain() -> miette::Result<BotQueue> {
    let concurrency = std::env::var("SHARD_CONCURRENCY")
        .into_diagnostic()?
        .parse::<usize>()
        .into_diagnostic()?;
    let wait = Duration::from_secs(
        std::env::var("SHARD_CONCURRENCY_WAIT_SECONDS")
            .into_diagnostic()?
            .parse::<u64>()
            .into_diagnostic()?,
    );

    Ok(if concurrency == 1 {
        BotQueue::Local(LocalQueue::new(wait))
    } else {
        BotQueue::LargeBot(LargeBotQueue::new(concurrency, wait))
    })
}
