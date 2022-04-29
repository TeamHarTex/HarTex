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

//! Handle a `GUILD_CREATE` event from the gateway.

use std::env as stdenv;

use base::discord::model::gateway::payload::incoming::GuildCreate;
use cache_discord::DiscordCache;
use manidb::whitelist::GetGuildWhitelistStatus;
use tide::http::headers::HeaderValue;
use tide::{Request, Response, Result};

/// Request handler for a `GUILD_CREATE` event.
///
/// The `GUILD_CREATE` event is received through the `/guild-create` endpoint.
pub async fn guild_create(mut request: Request<()>) -> Result<Response> {
    log::trace!("received guild create event payload from gateway, validating request...");
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

    log::trace!("deserializing guild create payload");
    let result = request.body_json::<GuildCreate>().await;
    if let Err(error) = result {
        log::error!("failed to deserialize guild create payload; see http error below");
        log::error!("http error: {error}; responding with the status of the error");
        return Ok(Response::new(error.status()));
    }

    let guild_create = result.unwrap();
    log::info!(
        "joined a new guild `{}` with id {}; checking its whitelist status",
        &guild_create.name,
        guild_create.id
    );

    let result = GetGuildWhitelistStatus::new(guild_create.id).await;
    if result.is_err() {
        log::trace!("internal error occurred, responding with HTTP 500");
        return Ok(Response::new(500));
    }

    if let Some(whitelist_status) = result.unwrap() {
        log::info!(
            "guild of id {} is whitelisted; and has been whitelisted since {}",
            whitelist_status.id(),
            whitelist_status.whitelisted_since()
        );
    } else {
        log::error!("guild of id is not whitelisted {}", guild_create.id);
    }

    log::trace!("caching guild");
    if let Err(error) = DiscordCache.update(&guild_create).await {
        log::trace!("failed to cache guild: {error:?}; responding with HTTP 500");
        return Ok(Response::new(500));
    }

    log::trace!("processing completed, responding with HTTP 200");

    Ok(Response::new(200))
}
