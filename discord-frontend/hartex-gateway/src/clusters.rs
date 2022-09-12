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
use std::sync::Arc;

use futures_util::Stream;
use hartex_core::discord::gateway::cluster::ShardScheme;
use hartex_core::discord::gateway::queue::Queue;
use hartex_core::discord::gateway::shard::ResumeSession;
use hartex_core::discord::gateway::{Cluster, Event, EventTypeFlags, Intents};
use hartex_core::log;

pub async fn get_clusters(
    num_shards: u64,
    queue: Arc<dyn Queue>,
    resume_sessions: HashMap<u64, ResumeSession>,
) -> hartex_eyre::Result<(
    Vec<Arc<Cluster>>,
    Vec<impl Stream<Item = (u64, Event)> + Send + Sync + Unpin + 'static>,
)> {
    let num_clusters = std::env::var("NUM_CLUSTERS")?.parse::<u64>()?;
    let num_base = num_shards / num_clusters;
    let num_extra = num_shards % num_clusters;

    let bot_token = std::env::var("BOT_TOKEN")?;

    let mut clusters = Vec::with_capacity(num_clusters as usize);
    let mut events = Vec::with_capacity(num_clusters as usize);
    let mut last_shard_index = std::env::var("SHARDS_START_INDEX")?.parse::<u64>()?;

    for i in 0..num_clusters {
        let index = if i < num_extra {
            last_shard_index + num_base
        } else {
            last_shard_index + num_base - 1
        };

        log::trace!("building cluster {i} with shards {last_shard_index}..{index}");
        let (cluster, event) = Cluster::builder(bot_token.clone(), Intents::all())
            .shard_scheme(ShardScheme::Range {
                from: last_shard_index,
                to: index,
                total: num_shards,
            })
            .queue(queue.clone())
            .event_types(EventTypeFlags::all())
            .resume_sessions(resume_sessions.clone())
            .build()
            .await?;
        clusters.push(Arc::new(cluster));
        events.push(event);

        last_shard_index = index + 1;
    }

    Ok((clusters, events))
}
