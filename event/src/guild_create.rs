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
use serde_json::json;
use tide::http::headers::HeaderValue;
use tide::{Request, Response, Result, StatusCode};

/// Request handler for a `READY` event.
///
/// The `GUILD_CREATE` event is received through the `/guild-create` endpoint.
pub async fn guild_create(mut request: Request<()>) -> Result<Response> {
    log::trace!("received guild create event payload from gateway, validating request...");
    let option = request.header("Authorization");
    if option.is_none() {
        log::error!("`Authorization` header was not found, responding with HTTP 401");

        return Ok(Response::builder(StatusCode::Unauthorized)
            .body_json(&json!({
                "code": 401i16,
                "message": "Unauthorized",
            }))
            .unwrap()
            .build());
    }

    let auth_header = option.unwrap();
    let result = stdenv::var("EVENT_SERVER_AUTH");
    let auth = if let Ok(auth) = result {
        auth
    } else {
        let error = result.unwrap_err();
        log::error!("env error: {error}; responding with HTTP 500");
        return Ok(Response::builder(StatusCode::InternalServerError)
            .body_json(&json!({
                "code": 500i16,
                "message": "Internal Server Error",
            }))
            .unwrap()
            .build());
    };

    if !auth_header.contains(&HeaderValue::from_bytes(auth.as_bytes().to_vec()).unwrap()) {
        log::error!("`Authorization` header does not contain the correct secret key; responding with HTTP 401");

        return Ok(Response::builder(StatusCode::Unauthorized)
            .body_json(&json!({
                "code": 401i16,
                "message": "Unauthorized",
            }))
            .unwrap()
            .build());
    }

    log::trace!("deserializing guild create payload");
    let result = request.body_json::<GuildCreate>().await;
    if let Err(error) = result {
        log::error!("failed to deserialize guild create payload; see http error below");
        log::error!("http error: {error}; responding with the status of the error");
        return Ok(Response::builder(error.status())
            .body_json(&json!({
                "code": error.status(),
                "message": error.status().canonical_reason(),
            }))
            .unwrap()
            .build());
    }

    let _ = result.unwrap();

    log::trace!("processing completed, responding with HTTP 200");

    Ok(Response::builder(StatusCode::Ok)
        .body_json(&json!({
            "code": 200i16,
            "message": "OK",
        }))
        .unwrap()
        .build())
}
