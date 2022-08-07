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

use std::collections::HashMap;
use std::env as stdenv;

use base::discord::http::routing::Route;
use base::error::{Error, ErrorKind, Result};
use ext::discord::model::application::command::HarTexCommand;
use hyper::{Body, Method, Request};
use loadbal::Request as LoadbalRequest;
use rest::request::RatelimitRequest;

pub fn create_global_application_command(
    command: HarTexCommand,
    loadbal_port: u16,
) -> Result<Request<Body>> {
    let result = stdenv::var("BOT_TOKEN");
    if let Err(src) = result {
        return Err(Error {
            kind: ErrorKind::EnvVarError { src },
        });
    }

    let mut token = result.unwrap();
    if !token.starts_with("Bot ") {
        token.insert_str(0, "Bot ");
    }

    let mut headers = HashMap::new();
    headers.insert(String::from("Authorization"), token);

    let serde_result = serde_json::to_string(&command);
    if let Err(src) = serde_result {
        return Err(Error {
            kind: ErrorKind::JsonError { src },
        });
    }
    let rl_request = RatelimitRequest {
        body: serde_result.unwrap(),
        method: Method::POST.to_string(),
        headers,
    };
    let serde_result = serde_json::to_string(&rl_request);
    if let Err(src) = serde_result {
        return Err(Error {
            kind: ErrorKind::JsonError { src },
        });
    }
    let body = serde_result.unwrap();

    let mut headers2 = HashMap::new();
    let result = stdenv::var("REST_SERVER_AUTH");
    if let Err(src) = result {
        return Err(Error {
            kind: ErrorKind::EnvVarError { src },
        });
    }

    headers2.insert(String::from("X-Authorization"), result.unwrap());

    #[cfg(stable)]
    let application_id = 936431574310879332;
    #[cfg(not(stable))]
    let application_id = 936432439767740436;

    let loadbal_request = LoadbalRequest {
        target_server_type: String::from("rest"),
        method: Method::POST.to_string(),
        route: Route::CreateGlobalCommand { application_id }.to_string(),
        headers: headers2,
        body,
    };
    let serde_result = serde_json::to_string(&loadbal_request);
    if let Err(src) = serde_result {
        return Err(Error {
            kind: ErrorKind::JsonError { src },
        });
    }
    let actual_json = serde_result.unwrap();

    let result = Request::builder()
        .method(Method::POST)
        .uri(format!("http://127.0.0.1:{loadbal_port}/request"))
        .body(Body::from(actual_json));
    if let Err(src) = result {
        return Err(Error {
            kind: ErrorKind::HttpError { src },
        });
    }

    Ok(result.unwrap())
}
