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

use base::error::Result;
use base::logging;
use env::{EnvVarKind, EnvVarValue, EnvVars};

mod ready;

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
    SyncLazy::force(&ENV);
    if ENV.is_none() {
        log::warn!("environment variables cannot be retrieved; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    logging::init();

    log::trace!("retrieving port to listen on");
    let port = match &ENV.as_ref().unwrap()["EVENT_SERVER_PORT"] {
        EnvVarValue::U16(port) => port,
        _ => unreachable!(),
    };

    log::trace!("creating http server");
    let mut server = tide::new();
    server.at("/ready").post(ready::ready);

    log::trace!("listening on port {port}");
    if let Err(error) = server.listen(format!("127.0.0.1:{port}")).await {
        log::error!("server error: could not listen on port {port}: {error}");
    }

    Ok(())
}
