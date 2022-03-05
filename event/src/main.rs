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
//!
//! ### `EVENT_SERVER_PORT`
//!
//! The port for the event HTTP server to listen on. This must be configured for the server to
//! start, and must be an integer.

#![deny(clippy::pedantic)]
#![deny(warnings)]

use std::env as stdenv;

use base::error::Result;
use base::logging;
use base::panicking;

mod guild_create;
mod ready;

#[tokio::main]
pub async fn main() -> Result<()> {
    logging::init();
    panicking::init();

    if let Err(error) = env::load() {
        log::error!("env error: {error}");
        log::warn!("environment variables cannot be loaded; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    log::trace!("retrieving port to listen on");
    let result = stdenv::var("EVENT_SERVER_PORT");
    let port = if let Ok(port) = result {
        let result = port.parse::<u16>();
        if let Ok(port) = result {
            port
        } else {
            log::error!(
                "processing error: port is not an integer: {}",
                result.unwrap_err()
            );
            return Ok(());
        }
    } else {
        log::error!("env error: {}", result.unwrap_err());
        return Ok(());
    };

    log::trace!("creating http server");
    let mut server = tide::new();
    server.at("/ready").post(ready::ready);
    server.at("/guild-create").post(guild_create::guild_create);

    log::trace!("listening on port {port}");
    if let Err(error) = server.listen(format!("127.0.0.1:{port}")).await {
        log::error!("server error: could not listen on port {port}: {error}");
    }

    Ok(())
}
