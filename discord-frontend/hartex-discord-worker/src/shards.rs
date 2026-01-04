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

use std::time::Duration;

use color_eyre::{Result, eyre::eyre};
use hartex_discord_grpc::manager::{ReadyResponse, WorkerSessionStartLimit};
use twilight_gateway::{ConfigBuilder, Shard, create_iterator, queue::InMemoryQueue};
use twilight_model::gateway::{
    Intents,
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};

pub async fn create(token: String, ready: ReadyResponse) -> Result<impl Iterator<Item = Shard>> {
    let Some(WorkerSessionStartLimit {
        max_concurrency,
        remaining,
        reset_after,
        total,
    }) = ready.session_start_limit
    else {
        return Err(eyre!("session start limit not specified"));
    };

    let config = ConfigBuilder::new(token, Intents::all())
        .queue(InMemoryQueue::new(
            max_concurrency as u16,
            remaining,
            Duration::from_secs(reset_after),
            total,
        ))
        .build();
    let total = ready.initial_assignments.len();
    let numbers = ready
        .initial_assignments
        .into_iter()
        .map(|assignment| assignment.shard_id);

    Ok(create_iterator(
        numbers,
        total as u32,
        config,
        |shard_id, builder| {
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
                    status: Status::Idle,
                })
                .build()
        },
    ))
}
