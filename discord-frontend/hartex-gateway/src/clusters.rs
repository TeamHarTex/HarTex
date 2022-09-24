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

use futures_util::StreamExt;
use hartex_core::discord::gateway::{stream, Config, EventTypeFlags, Intents, Shard};
use hartex_core::log;

pub async fn get_clusters(num_shards: u64) -> hartex_eyre::Result<Vec<Vec<Shard>>> {
    let num_clusters = std::env::var("NUM_CLUSTERS")?.parse::<u64>()?;
    let num_base = num_shards / num_clusters;
    let num_extra = num_shards % num_clusters;

    let bot_token = std::env::var("BOT_TOKEN")?;

    let mut last_shard_index = std::env::var("SHARDS_START_INDEX")?.parse::<u64>()?;

    let mut clusters = Vec::with_capacity(num_clusters as usize);
    for i in 0..num_clusters {
        let index = if i < num_extra {
            last_shard_index + num_base
        } else {
            last_shard_index + num_base - 1
        };

        log::trace!("building cluster {i} with shards {last_shard_index}..{index}");
        let cluster = stream::start_cluster(last_shard_index, 1, index, |shard_id| {
            Config::builder(bot_token.clone(), Intents::all())
                .event_types(EventTypeFlags::all())
                .build()
        })
        .filter_map(|shard| async move { shard.ok() })
        .collect::<Vec<_>>()
        .await;

        clusters.push(cluster);
        last_shard_index = index + 1;
    }

    Ok(clusters)
}
