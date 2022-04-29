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

use std::str::FromStr;
use std::time::Duration;

use hyper::body;
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Request as HyperRequest};
use rest::request::RatelimitRequest;
use rest::RestState;
use serde_json::json;
use tide::{Request, Response, Result};
use tokio::time;
use url::Position;

#[allow(clippy::module_name_repetitions)]
pub async fn proxy_request(mut request: Request<RestState>) -> Result<Response> {
    log::trace!("received request to forward to Discord API");
    log::trace!("deserializing request payload");
    let result = request.body_json::<RatelimitRequest>().await;
    if let Err(error) = result {
        log::error!("failed to deserialize proxy request payload; see http error below");
        log::error!("http error: {error}; responding with the status of the error");
        return Ok(Response::builder(error.status())
            .body_json(&json!({
                "code": error.status(),
                "message": error.status().canonical_reason(),
            }))
            .unwrap()
            .build());
    }

    let path = &request.url()[Position::BeforePath..];
    let rl_request = result.unwrap();

    log::trace!("attempting to resolve ratelimits and make request");
    loop {
        let _ = request.state().ratelimit.global.lock().await;

        let entry = request.state().ratelimit.buckets.write().await;
        let entry = entry.entry(path.to_string()).or_default();
        let value = entry.value().clone();
        value.lock().await.preflight().await;

        let mut builder = HyperRequest::builder()
            .uri(format!("https://discord.com/api/v10{path}"))
            .method(rl_request.method());
        let headers = builder.headers_mut().unwrap();
        for (name, value) in rl_request.headers() {
            headers.insert(
                HeaderName::from_str(&name).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }
        let hyper = builder
            .body(Body::from(rl_request.body().to_string()))
            .unwrap();

        let client = request.state().client.clone();
        let result = client.request(hyper).await;

        if let Err(error) = result {
            log::error!("failed to make request: {error}");
            return Ok(Response::new(400));
        };

        let mut response = result.unwrap();

        let retry = if response.headers().get("X-RateLimit-Global").is_some() {
            let _ = request.state().ratelimit.global.lock().await;

            if let Some(value) = response.headers().get("Retry-After") {
                let str = value.to_str().unwrap();
                let retry_after = str.parse::<f64>().unwrap();

                time::sleep(Duration::from_secs_f64(retry_after)).await;

                true
            } else {
                false
            }
        } else {
            value.lock().await.postflight(&response).await
        };

        if !retry {
            let mut ret = Response::new(200);
            let bytes = body::to_bytes(response.body_mut()).await.unwrap();
            ret.body_bytes(bytes);

            return Ok(ret);
        }
    }
}
