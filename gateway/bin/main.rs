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

#![feature(let_else)]

use std::env as stdenv;

use async_std::channel;
use async_std::task;
use base::cmdline;
use base::discord::gateway::cluster::ClusterStartErrorType;
use base::discord::gateway::{Cluster, Event, EventTypeFlags, Intents};
use base::discord::model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use base::discord::model::gateway::presence::{Activity, ActivityType, Status};
use base::error::Result;
use base::logging;
use base::panicking;
use env;
use ext::discord::model::gateway::event::EventExt;
use futures_util::StreamExt;
use tide_websockets::WebSocket;

// FIXME: avoid receiving all events and using all gateway intents
const EVENT_TYPE_FLAGS: EventTypeFlags = EventTypeFlags::all();
const GATEWAY_INTENTS: Intents = Intents::all();

#[async_std::main]
pub async fn main() -> Result<()> {
    logging::init();
    panicking::init();

    let args = stdenv::args().collect::<Vec<_>>();
    let args = &args[1..];
    let mut base_options = cmdline::Options::new();
    let options = base_options.reqopt(
        "",
        "port",
        "The port for the gateway server to run on",
        "PORT",
    );

    let Some(matches) = options.parse(args).ok() else {
        log::error!("could not parse command line arguments; exiting");

        return Ok(());
    };
    let Ok(Some(port)) = matches.opt_get::<u16>("port") else {
        log::error!("could not parse port argument; exiting");

        return Ok(());
    };

    if let Err(error) = env::load() {
        log::error!("env error: {error}");
        log::warn!("environment variables cannot be loaded; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    let (tx, rx) = channel::unbounded::<(u64, Event)>();

    log::trace!("creating websocket server");
    let mut server = tide::new();
    server
        .at("/ws")
        .get(WebSocket::new(move |_request, _connection| {
            let new_rx = rx.clone();

            async {
                let task = task::spawn(async move {
                    while let Ok((_shard, _event)) = new_rx.recv().await {
                        todo!()
                    }

                    Ok(())
                });
                task.await
            }
        }));

    if task::spawn(async {
        let result = stdenv::var("BOT_TOKEN");
        let token = if let Ok(token) = result {
            token
        } else {
            log::error!("env error: {}", result.unwrap_err());
            return Err(());
        };

        log::trace!("creating gateway cluster");
        let result = Cluster::builder(token, GATEWAY_INTENTS)
            .event_types(EVENT_TYPE_FLAGS)
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
                        Status::Idle,
                    )
                    .unwrap(),
                )
            })
            .build()
            .await;

        if let Err(error) = &result {
            log::warn!("cluster could not be built; exiting");

            let reason = match error.kind() {
                ClusterStartErrorType::RetrievingGatewayInfo => {
                    "gateway information retrieval failed"
                }
                ClusterStartErrorType::Tls => "tls connector creation failed",
                _ => "unknown error",
            };
            log::error!(r#"this is due to "{reason}""#);

            return Err(());
        }

        let (cluster, mut events) = result.unwrap();

        task::spawn(async move {
            cluster.up().await;
        });

        while let Some((shard_id, event)) = events.next().await {
            let new_tx = tx.clone();

            log::trace!(
                "shard {shard_id} received an event of type {} from the discord gateway",
                event.as_str()
            );

            new_tx.send((shard_id, event)).await.unwrap()
        }

        Ok(())
    })
    .await
    .is_err()
    {
        return Ok(());
    }

    log::trace!("listening on port {port}");
    if let Err(error) = server.listen(format!("127.0.0.1:{port}")).await {
        log::error!("server error: could not listen on port {port}: {error}");
    }

    Ok(())
}
