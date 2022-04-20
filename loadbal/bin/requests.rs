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

use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Client, Method, Request as HyperRequest, Uri};
use loadbal::Request as LoadbalRequest;
use serde_json::json;
use tide::{Request, Response, Result};

use crate::servers;

pub async fn handle_request(mut request: Request<()>) -> Result<Response> {
    log::trace!("received a request into load balancer, processing request");
    log::trace!("deserializing request payload");
    let result = request.body_json::<LoadbalRequest>().await;
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

    let loadbal_request = result.unwrap();

    log::trace!("retrieving all possible servers");
    let target = loadbal_request.target_server_type();
    let target_ips = servers::SERVERS
        .iter()
        .filter(|entry| entry.key() == &target)
        .map(|entry| entry.value().clone())
        .collect::<Vec<_>>();
    if let Some(ip) = get_good_ip(target_ips).await {
        log::trace!("building http request to actual server");
        log::trace!(
            "route: {}",
            format!("http://{}/{}", &ip, loadbal_request.route())
        );
        let mut builder = HyperRequest::builder()
            .uri(format!("http://{}/{}", ip, loadbal_request.route()))
            .method(loadbal_request.method());
        let headers = builder.headers_mut().unwrap();
        for (name, value) in loadbal_request.headers() {
            headers.insert(
                HeaderName::from_str(&name).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        let request = builder
            .body(Body::from(loadbal_request.body().to_string()))
            .unwrap();

        log::trace!("sending request");
        if Client::new().request(request).await.is_err() {
            log::trace!("request failed, responding with HTTP 503");
            return Ok(Response::new(503));
        };

        Ok(Response::new(200))
    } else {
        log::trace!("no server available, responding with HTTP 503");
        Ok(Response::new(503))
    }
}

async fn get_good_ip(all: Vec<Uri>) -> Option<Uri> {
    log::trace!("retrieving available servers");

    for ip in all {
        log::trace!("pinging {ip}");

        let request = HyperRequest::builder()
            .method(Method::POST)
            .uri(format!("http://{ip}/ping"))
            .body(Body::empty())
            .unwrap();

        if Client::new().request(request).await.is_ok() {
            log::trace!("server at {ip} is available");

            return Some(ip);
        }
    }

    None
}
