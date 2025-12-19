/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use hartex_discord_utils::{CLIENT, TOKEN, error::HarTexResult};
use twilight_gateway::{ConfigBuilder, Shard, create_recommended};
use twilight_model::gateway::{
    Intents,
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};

pub async fn create() -> HarTexResult<Vec<Shard>> {
    let config = ConfigBuilder::new(TOKEN.clone()?, Intents::all()).build();

    Ok(create_recommended(&CLIENT, config, |shard_id, builder| {
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
    })
    .await?
    .collect())
}
