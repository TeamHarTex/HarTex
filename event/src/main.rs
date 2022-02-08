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
use std::net::{SocketAddr};

use base::error::{Result};
use base::logging;
use env::{EnvVarKind, EnvVarValue, EnvVars};
use hyper::server::Server;

static ENV: SyncLazy<Option<EnvVars>> = SyncLazy::new(|| {
    log::trace!("retrieving environment variables");

    let result = EnvVars::get(EnvVarKind::Common);
    if let Err(error) = &result {
        log::error!("retrieval failed: `{error}`");
    }

    result.ok()
});

#[tokio::main]
pub async fn main() -> Result<()> {
    logging::init();

    SyncLazy::force(&ENV);
    if ENV.is_none() {
        log::warn!("environment variables cannot be retrieved; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    let port = match &ENV.as_ref().unwrap()["EVENT_SERVER_PORT"] {
        EnvVarValue::U16(port) => port,
        _ => unreachable!(),
    };
    let addr: SocketAddr = ([127, 0, 0, 1], *port).into();
    let server = Server::bind(&addr).http2_only(true);



    Ok(())
}
