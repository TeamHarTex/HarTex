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

use std::collections::HashMap;
use std::env as stdenv;

use base::discord::model::gateway::payload::incoming::GuildCreate;
use cache_discord::DiscordCache;
use hyper::{Body, Client, Method, Request as Hyper};
use loadbal::Request as LoadbalRequest;
use manidb::whitelist::GetGuildWhitelistStatus;
use rest::request::RatelimitRequest;
use tide::http::headers::HeaderValue;
use tide::{Request, Response, Result};

/// Request handler for a `GUILD_CREATE` event.
///
/// The `GUILD_CREATE` event is received through the `/guild-create` endpoint.
pub async fn guild_create(mut request: Request<u16>) -> Result<Response> {
    log::trace!("received guild create event payload from gateway, validating request...");
    let option = request.header("Authorization");
    if option.is_none() {
        log::error!("`Authorization` header was not found, responding with HTTP 401");

        return Ok(Response::new(401));
    }

    let auth_header = option.unwrap();
    let result = stdenv::var("EVENT_SERVER_AUTH");
    let Ok(auth) = result else {
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
        log::error!(
            "guild of id {} is not whitelisted, leaving guild",
            guild_create.id
        );

        let mut headers = HashMap::new();

        let result = stdenv::var("BOT_TOKEN");
        let Ok(mut token) = result else {
            let error = result.unwrap_err();
            log::error!("env error: {error}; responding with HTTP 500");
            return Ok(Response::new(500));
        };
        token.insert_str(0, "Bot ");
        headers.insert(String::from("Authorization"), token);

        let rl_request = RatelimitRequest {
            method: "DELETE".to_string(),
            headers,
            body: "".to_string()
        };
        let serde_result = serde_json::to_string(&rl_request);
        let Ok(body) = serde_result else {
            log::error!("failed to serialize ratelimited request; responding with HTTP 500");
            return Ok(Response::new(500));
        };

        let loadbal_request = LoadbalRequest {
            target_server_type: "rest".to_string(),
            method: "POST".to_string(),
            route: format!("users/@me/guilds/{}", guild_create.id),
            headers: HashMap::new(),
            body
        };

        let serde_result = serde_json::to_string(&loadbal_request);
        let result = Hyper::builder()
            .method(Method::POST)
            .uri(format!("http://127.0.0.1:{}/request", request.state()))
            .body(Body::from(serde_result.unwrap()));
        if let Err(src) = result {
            log::error!("request error: could not build request: {src}; responding with HTTP 500");
            return Ok(Response::new(500));
        }

        let request = result.unwrap();

        log::trace!("sending request");
        tokio::spawn(async move {
            let client = Client::new();
            client.request(request).await
        });

        log::trace!("processing completed, responding with HTTP 200");
        return Ok(Response::new(200));
    }

    log::trace!("caching guild");
    if let Err(error) = DiscordCache.update(&guild_create).await {
        log::trace!("failed to cache guild: {error:?}; responding with HTTP 500");
        return Ok(Response::new(500));
    }

    log::trace!("processing completed, responding with HTTP 200");

    Ok(Response::new(200))
}
