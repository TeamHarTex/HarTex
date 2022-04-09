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

//! Helper functions to send an HTTP request to the event HTTP server when an event is received
//! from the Discord gateway.

use std::env as stdenv;

use base::discord::model::gateway::event::Event;
use base::error::{Error, ErrorKind, Result};
use hyper::header::AUTHORIZATION;
use hyper::{Body, Client, Method, Request};

/// Send an HTTP request containing the corresponding gateway event payload to the event HTTP
/// server for further processing.
pub async fn emit_event(event: Event, port: u16) -> Result<()> {
    let result = stdenv::var("EVENT_SERVER_AUTH");
    let auth = if let Ok(ref auth) = result {
        auth
    } else {
        let error = result.unwrap_err();
        log::error!("env error: {error}");
        return Err(Error::from(error));
    };

    let request = match event {
        Event::GuildCreate(guild_create) => {
            log::trace!("serializing guild create payload");
            let serde_result = serde_json::to_string(&guild_create);
            if let Err(src) = serde_result {
                log::error!("request error: could not serialize body: {src}");
                return Err(Error::from(ErrorKind::JsonError { src }));
            }

            log::trace!("building request");
            let result = Request::builder()
                .header(AUTHORIZATION, auth)
                .method(Method::POST)
                .uri(format!("http://127.0.0.1:{port}/guild-create"))
                .body(Body::from(serde_result.unwrap()));
            if let Err(src) = result {
                log::error!("request error: could not build request: {src}");
                return Err(Error::from(ErrorKind::HttpError { src }));
            }

            Some(result.unwrap())
        }
        Event::Ready(ready) => {
            log::trace!("serializing ready payload");
            let serde_result = serde_json::to_string(&ready);
            if let Err(src) = serde_result {
                log::error!("request error: could not serialize body: {src}");
                return Err(Error::from(ErrorKind::JsonError { src }));
            }

            log::trace!("building request");
            let result = Request::builder()
                .header(AUTHORIZATION, auth)
                .method(Method::POST)
                .uri(format!("http://127.0.0.1:{port}/ready"))
                .body(Body::from(serde_result.unwrap()));
            if let Err(src) = result {
                log::error!("request error: could not build request: {src}");
                return Err(Error::from(ErrorKind::HttpError { src }));
            }

            Some(result.unwrap())
        }
        _ => None,
    };

    if request.is_none() {
        return Ok(());
    }

    log::trace!("adding request to queue");
    tokio::spawn(async move {
        let client = Client::new();
        client.request(request.unwrap()).await
    });

    Ok(())
}
