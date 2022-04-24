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

//! Handle a `READY` event from the gateway.

use std::env as stdenv;

use base::discord::model::gateway::payload::incoming::Ready;
use cache_discord::DiscordCache;
use tide::http::headers::HeaderValue;
use tide::{Request, Response, Result};

/// Request handler for a `READY` event.
///
/// The `READY` event is received through the `/ready` endpoint.
pub async fn ready(mut request: Request<()>) -> Result<Response> {
    log::trace!("received ready event payload from gateway, validating request...");
    let option = request.header("Authorization");
    if option.is_none() {
        log::error!("`Authorization` header was not found, responding with HTTP 401");

        return Ok(Response::new(401));
    }

    let auth_header = option.unwrap();
    let result = stdenv::var("EVENT_SERVER_AUTH");
    let auth = if let Ok(auth) = result {
        auth
    } else {
        let error = result.unwrap_err();
        log::error!("env error: {error}; responding with HTTP 500");
        return Ok(Response::new(500));
    };

    if !auth_header.contains(&HeaderValue::from_bytes(auth.as_bytes().to_vec()).unwrap()) {
        log::error!("`Authorization` header does not contain the correct secret key; responding with HTTP 401");

        return Ok(Response::new(401));
    }

    log::trace!("deserializing ready payload");
    let result = request.body_json::<Ready>().await;
    if let Err(error) = result {
        log::error!("failed to deserialize ready payload; see http error below");
        log::error!("http error: {error}; responding with the status of the error");
        return Ok(Response::new(error.status()));
    }

    let ready = result.unwrap();
    let shard_info = ready.shard.unwrap();
    log::trace!(
        "shard {} received ready event from the gateway",
        shard_info[0],
    );
    log::info!(
        "{}#{} [uid {}] is connected to the discord gateway and is ready; the shard uses gateway version {} (shard {})",
        ready.user.name,
        ready.user.discriminator,
        ready.user.id,
        ready.version,
        shard_info[0],
    );

    log::trace!("caching current user");
    if let Err(error) = DiscordCache.update(&ready).await {
        log::trace!("failed to cache current user: {error:?}; responding with HTTP 500");
        return Ok(Response::new(500));
    }

    log::trace!("processing completed, responding with HTTP 200");

    Ok(Response::new(200))
}
