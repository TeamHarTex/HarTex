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

use base::discord::model::gateway::payload::incoming::Ready;
use env::EnvVarValue;
use serde_json::json;
use tide::http::headers::HeaderValue;
use tide::{Request, Response, Result, StatusCode};

use crate::ENV;

pub async fn ready(mut request: Request<()>) -> Result<Response> {
    let option = request.header("Authorization");
    if option.is_none() {
        return Ok(Response::builder(StatusCode::Unauthorized)
            .body_json(&json!({
                "code": 401,
                "message": "Unauthorized",
            }))
            .unwrap()
            .build());
    }

    let auth_header = option.unwrap();
    let auth = match &ENV.as_ref().unwrap()["EVENT_SERVER_AUTH"] {
        EnvVarValue::String(auth) => auth,
        _ => unreachable!(),
    };

    if !auth_header.contains(&HeaderValue::from_bytes(auth.as_bytes().to_vec()).unwrap()) {
        return Ok(Response::builder(StatusCode::Unauthorized)
            .body_json(&json!({
                "code": 401,
                "message": "Unauthorized",
            }))
            .unwrap()
            .build());
    }

    log::trace!("deserializing ready payload");
    let result = request.body_json::<Ready>().await;
    if let Err(error) = result {
        log::error!("failed to deserialize ready payload; see http error below");
        log::error!("http error: {error}");
        return Ok(Response::builder(error.status())
            .body_json(&json!({
                "code": error.status(),
                "message": error.status().canonical_reason(),
            }))
            .unwrap()
            .build());
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

    Ok(Response::builder(StatusCode::Ok)
        .body_json(&json!({
            "code": 200,
            "message": "OK",
        }))
        .unwrap()
        .build())
}
