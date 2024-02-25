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

use std::ops::Deref;

use hartex_discord_core::discord::gateway::create_recommended;
use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::discord::gateway::ConfigBuilder;
use hartex_discord_core::discord::gateway::Intents;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::discord::gateway::ShardId;
use hartex_discord_core::discord::model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use hartex_discord_core::discord::model::gateway::presence::Activity;
use hartex_discord_core::discord::model::gateway::presence::ActivityType;
use hartex_discord_core::discord::model::gateway::presence::Status;
use hartex_discord_utils::CLIENT;
use hartex_discord_utils::TOKEN;
use miette::IntoDiagnostic;

/// Obtain a list of shards.
pub async fn obtain<Q>(queue: Q) -> miette::Result<Vec<Shard<Q>>>
where
    Q: Queue + Clone + Send + Sync + Sized,
{
    let config = ConfigBuilder::new(TOKEN.deref().clone(), Intents::all())
        .queue(queue.clone())
        .build();

    Ok(create_recommended::<_, Q>(
        &CLIENT,
        config,
        |shard_id: ShardId, builder: ConfigBuilder<Q>| {
            builder
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
        },
    )
    .await
    .into_diagnostic()?
    .collect::<Vec<_>>())
}
