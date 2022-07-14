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

//! # The HarTex Event Process Binary
//!
//! The HarTex Event Process Binary sets up an HTTP server, and receives HTTP requests as the
//! standalone gateway process receives events emitted by the Discord gateway.
//!
//! There are various environment variables that needs to be configured for the event HTTP server
//! to operate correctly. The list of environment variables are listed below.
//!
//! ## Environment Variables Required
//!
//! ### `EVENT_SERVER_AUTH`
//!
//! A secret key expected as the `Authorization` header with incoming HTTP requests to validate the
//! requests.

#![deny(clippy::pedantic)]
#![deny(warnings)]
#![feature(let_else)]
// allow match_result_ok lint
#![allow(clippy::match_result_ok)]

use std::env as stdenv;

use async_tungstenite::tokio as tokio_tungstenite;
use async_tungstenite::tungstenite::Message;
use base::cmdline;
use base::error::Result;
use base::logging;
use base::panicking;
use futures_util::StreamExt;
use gateway::Payload;

mod payload;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<()> {
    logging::init();
    panicking::init();

    let args = stdenv::args().collect::<Vec<_>>();
    let args = &args[1..];
    let mut base_options = cmdline::Options::new();
    let options = base_options.reqopt(
        "",
        "gateway-port",
        "The port of the gateway server ",
        "PORT",
    );

    if args.is_empty() {
        event_usage(options);
        return Ok(());
    }

    let result = options.parse(args);
    if let Err(error) = result {
        match error {
            cmdline::Fail::UnrecognizedOption(option) => {
                println!("event: unrecognized option: {option}");
            }
            cmdline::Fail::OptionMissing(option) => {
                println!("event: missing required option: {option}");
            }
            _ => (),
        }

        return Ok(());
    }
    let matches = result.unwrap();

    let Ok(Some(gateway_port)) = matches.opt_get::<u16>("gateway-port") else {
        log::error!("could not parse gateway port argument; exiting");

        return Ok(());
    };

    if let Err(error) = env::load() {
        log::error!("env error: {error}");
        log::warn!("environment variables cannot be loaded; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    log::trace!("attempting to connect to gateway...");
    let result =
        tokio_tungstenite::connect_async(&format!("ws://127.0.0.1:{gateway_port}/ws")).await;
    if let Err(error) = result {
        log::error!("connect error: failed to connect to gateway: {error}");

        return Ok(());
    }

    let (mut connection, _) = result.unwrap();
    log::trace!("successfully connected to gateway");

    while let Some(result) = connection.next().await {
        if let Ok(message) = result {
            match message {
                Message::Text(json) => {
                    tokio::spawn(payload::handle_payload(
                        serde_json::from_str::<Payload>(&json).unwrap(),
                    ));
                },
                _ => (),
            }
        }
    }

    Ok(())
}

pub fn event_usage(options: &mut cmdline::Options) {
    println!("{}", options.usage("Usage: event [options] [args...]"));
}
