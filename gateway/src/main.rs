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

//! # The HarTex Gateway Process Binary
//!
//! The HarTex Gateway Process Binary connects to the Discord gateway.
//!
//! There are various environment variables that needs to be set to send requests to the event HTTP
//! server correctly. The list of environment variables are listed below.
//!
//! ## Environment Variables Required
//!
//! ### `EVENT_SERVER_AUTH`
//!
//! A secret key to be passed as the `Authorization` header with outgoing HTTP requests for the
//! event HTTP server requests.

#![deny(clippy::pedantic)]
#![deny(warnings)]
#![feature(let_else)]
#![feature(once_cell)]

use std::env as stdenv;

use base::cmdline;
use base::discord::gateway::cluster::{ClusterStartErrorType, ShardScheme};
use base::discord::gateway::{Cluster, EventTypeFlags, Intents};
use base::discord::model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use base::discord::model::gateway::presence::{Activity, ActivityType, Status};
use base::error::Result;
use base::logging;
use base::panicking;
use ext::discord::model::gateway::event::EventExt;
use futures_util::StreamExt;

mod request;

const EVENT_TYPE_FLAGS: EventTypeFlags = EventTypeFlags::all();

const INTENTS: Intents = Intents::all();

#[allow(clippy::too_many_lines)]
#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<()> {
    logging::init();
    panicking::init();

    let args = stdenv::args().collect::<Vec<_>>();
    let args = &args[1..];
    let mut base_options = cmdline::Options::new();
    let options = base_options.reqopt(
        "",
        "port",
        "The port for gateway process to send events to for further processing",
        "PORT",
    );

    if args.is_empty() {
        gateway_usage(options);
        return Ok(());
    }

    if let Err(error) = env::load() {
        log::error!("env error: {error}");
        log::warn!("environment variables cannot be loaded; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    let result = options.parse(args);
    if let Err(error) = result {
        match error {
            cmdline::Fail::UnrecognizedOption(option) => {
                println!("gateway: unrecognized option: {option}");
            }
            cmdline::Fail::OptionMissing(option) => {
                println!("gateway: missing required option: {option}");
            }
            _ => (),
        }

        return Ok(());
    }
    let matches = result.unwrap();

    let Ok(Some(port)) = matches.opt_get::<u16>("port") else {
        log::error!("could not parse port argument; exiting");

        return Ok(());
    };

    let result = stdenv::var("BOT_TOKEN");
    let token = if let Ok(token) = result {
        token
    } else {
        log::error!("env error: {}", result.unwrap_err());
        return Ok(());
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

    tokio::spawn(async move {
        cluster.up().await;
    });

    while let Some((shard_id, event)) = events.next().await {
        log::trace!(
            "shard {shard_id} received an event of type {} from the discord gateway",
            event.as_str()
        );

        tokio::spawn(request::emit_event(event, port));
    }

    Ok(())
}

pub fn gateway_usage(options: &mut cmdline::Options) {
    println!("{}", options.usage("Usage: gateway [options] [args...]"));
}
