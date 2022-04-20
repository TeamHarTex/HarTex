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

use hyper::{Body, Client, Method, Request as HyperRequest, Uri};
use loadbal::Request as LoadbalRequest;
use serde_json::json;
use tide::{Request, Response, Result};

use crate::servers;

pub async fn handle_request(mut request: Request<()>) -> Result<Response> {
    log::trace!("received a request into load balancer, processing request");
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
    let target = loadbal_request.target_server_type();
    let target_ips = servers::SERVERS
        .iter()
        .filter(|entry| entry.key() == &target)
        .map(|entry| entry.value().clone())
        .collect::<Vec<_>>();
    if let Some(_) = get_good_ip(target_ips).await {
        todo!()
    } else {
        Ok(Response::new(503))
    }
}

async fn get_good_ip(all: Vec<Uri>) -> Option<Uri> {
    for ip in all {
        let request = HyperRequest::builder()
            .method(Method::POST)
            .uri(format!("http://{ip}/ping"))
            .body(Body::empty())
            .unwrap();

        if let Ok(_) = Client::new().request(request).await {
            return Some(ip);
        }
    }

    None
}
