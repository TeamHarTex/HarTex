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

//! # The HarTex REST Process Binary
//!
//! The REST process binary acts as a proxy over the Discord API.

use std::env as stdenv;

use base::error::Result;
use base::logging;
use base::panicking;
use hyper::{Body, Client};
use hyper_rustls::HttpsConnectorBuilder;
use hyper_trust_dns::{TrustDnsHttpConnector, TrustDnsResolver};

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
    let result = stdenv::var("REST_SERVER_PORT");
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

    log::trace!("creating https connector for http client");
    let https_connector = {
        let mut connector = TrustDnsHttpConnector::new_with_resolver(TrustDnsResolver::new());
        connector.enforce_http(false);

        let builder = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1();

        if let Ok(enabled) = stdenv::var("REST_HTTP_2_ENABLED") && matches!(&enabled, "true") {
            builder.enable_http2().wrap_connector(connector)
        } else {
            builder.wrap_connector(connector)
        }
    };

    log::trace!("creating http client");
    let client = Client::builder().build::<_, Body>(https_connector);

    Ok(())
}
