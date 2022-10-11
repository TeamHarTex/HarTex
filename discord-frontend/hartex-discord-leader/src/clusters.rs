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

use std::sync::Arc;

use futures_util::StreamExt;
use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::discord::gateway::{stream, Config, EventTypeFlags, Intents, Shard};
use hartex_discord_core::discord::model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use hartex_discord_core::discord::model::gateway::presence::{Activity, ActivityType, Status};
use hartex_discord_core::log;

pub async fn get_clusters(
    num_shards: u64,
    queue: Arc<dyn Queue>,
) -> hartex_discord_eyre::Result<Vec<(u64, Vec<Shard>)>> {
    let num_clusters = std::env::var("NUM_CLUSTERS")?.parse::<u64>()?;
    let shard_per_cluster = num_shards.div_ceil(num_clusters);
    let remaining_shards = num_shards % num_clusters;

    let bot_token = std::env::var("BOT_TOKEN")?;

    let mut shard_start_index = std::env::var("SHARDS_START_INDEX")?.parse::<u64>()?;

    let mut clusters = Vec::with_capacity(num_clusters as usize);
    for i in 0..num_clusters {
        let total = shard_start_index + shard_per_cluster;

        log::trace!("building cluster {i} with shards {shard_start_index}..{total}");
        let cluster = stream::start_cluster(shard_start_index, 1, total, |shard_id| {
            Config::builder(bot_token.clone(), Intents::all())
                .event_types(EventTypeFlags::all())
                .presence(UpdatePresencePayload {
                    activities: vec![Activity {
                        application_id: None,
                        assets: None,
                        buttons: vec![],
                        created_at: None,
                        details: None,
                        emoji: None,
                        flags: None,
                        id: None,
                        instance: None,
                        kind: ActivityType::Watching,
                        name: format!("development | shard {}", shard_id.number()),
                        party: None,
                        secrets: None,
                        state: None,
                        timestamps: None,
                        url: None,
                    }],
                    afk: false,
                    since: None,
                    status: Status::Online,
                })
                .queue(queue.clone())
                .build()
        })
        .filter_map(|result| async move { result.ok() })
        .collect::<Vec<_>>()
        .await;
        clusters.push((i, cluster));
        shard_start_index = total;
    }

    if remaining_shards > 0 {
        log::trace!(
            "building cluster {num_clusters} with shards {shard_start_index}..{remaining_shards}"
        );
        let concurrency = std::env::var("SHARD_CONCURRENCY")?.parse()?;
        let cluster =
            stream::start_cluster(shard_start_index, concurrency, remaining_shards, |_| {
                Config::builder(bot_token.clone(), Intents::all())
                    .event_types(EventTypeFlags::all())
                    .queue(queue.clone())
                    .build()
            })
            .filter_map(|result| async move { result.ok() })
            .collect::<Vec<_>>()
            .await;
        clusters.push((num_clusters, cluster));
    }

    Ok(clusters)
}
