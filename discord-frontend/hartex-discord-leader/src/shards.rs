/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

use std::sync::Arc;

use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::discord::gateway::stream;
use hartex_discord_core::discord::gateway::Config;
use hartex_discord_core::discord::gateway::EventTypeFlags;
use hartex_discord_core::discord::gateway::Intents;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::discord::model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use hartex_discord_core::discord::model::gateway::presence::Activity;
use hartex_discord_core::discord::model::gateway::presence::ActivityType;
use hartex_discord_core::discord::model::gateway::presence::Status;

pub fn obtain(num_shards: u64, queue: &Arc<dyn Queue>) -> hartex_discord_eyre::Result<Vec<Shard>> {
    let bot_token = std::env::var("BOT_TOKEN")?;

    let shard_start_index = std::env::var("SHARDS_START_INDEX")?.parse::<u64>()?;

    Ok(
        stream::create_bucket(shard_start_index, 1, num_shards, |shard_id| {
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
        .collect::<Vec<_>>(),
    )
}
