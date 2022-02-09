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

#![feature(once_cell)]

use std::lazy::SyncLazy;

use base::discord::gateway::cluster::{ClusterStartErrorType, ShardScheme};
use base::discord::gateway::{Cluster, EventTypeFlags, GatewayCluster, Intents};
use base::discord::model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use base::discord::model::gateway::presence::{Activity, ActivityType, Status};
use base::error::Result;
use base::logging;
use env::{EnvVarKind, EnvVarValue, EnvVars};
use ext::discord::model::gateway::event::EventExt;
use futures_util::StreamExt;

mod request;

const EVENT_TYPE_FLAGS: EventTypeFlags = EventTypeFlags::all();

const INTENTS: Intents = Intents::all();

pub(in crate) static ENV: SyncLazy<Option<EnvVars>> = SyncLazy::new(|| {
    log::trace!("retrieving environment variables");

    let result = EnvVars::get(EnvVarKind::Common);
    if let Err(error) = &result {
        log::error!("retrieval failed: `{error}`");
    }

    result.ok()
});

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<()> {
    logging::init();

    SyncLazy::force(&ENV);
    if ENV.is_none() {
        log::warn!("environment variables cannot be retrieved; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    let token = match &ENV.as_ref().unwrap()["BOT_TOKEN"] {
        EnvVarValue::String(token) => token,
        _ => unreachable!(),
    };

    let result = Cluster::builder(token.clone(), INTENTS)
        .event_types(EVENT_TYPE_FLAGS)
        .shard_scheme(ShardScheme::Auto)
        .shard_presence(|id| {
            Some(
                UpdatePresencePayload::new(
                    vec![Activity {
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
                        name: format!("development | shard {id}"),
                        party: None,
                        secrets: None,
                        state: None,
                        timestamps: None,
                        url: None,
                    }],
                    false,
                    None,
                    Status::Online,
                )
                .unwrap(),
            )
        })
        .build()
        .await;
    if let Err(error) = &result {
        log::warn!("cluster could not be built; exiting");

        let reason = match error.kind() {
            ClusterStartErrorType::RetrievingGatewayInfo => "gateway information retrieval failed",
            ClusterStartErrorType::Tls => "tls connector creation failed",
            _ => "unknown error",
        };
        log::error!(r#"this is due to "{reason}""#);

        return Ok(());
    }

    let (cluster, mut events) = result.unwrap();
    let cluster = GatewayCluster::new(cluster);

    let spawn = cluster.clone();

    tokio::spawn(async move {
        spawn.up().await;
    });

    while let Some((shard_id, event)) = events.next().await {
        log::trace!(
            "shard {shard_id} received an event of type {} from the discord gateway",
            event.as_str()
        );
    }

    Ok(())
}
