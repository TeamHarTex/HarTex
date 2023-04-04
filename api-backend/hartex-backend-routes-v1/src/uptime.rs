/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;

use chrono::Duration;
use chrono::Utc;
use hartex_backend_models_v1::uptime::UptimeQuery;
use hartex_backend_ratelimiter::RateLimiter;
use hartex_backend_status_util::StatusFns;
use rocket::post;
use rocket::serde::json::Json;
use rocket::http::Status;
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::Compression;
use scylla::SessionBuilder;
use serde_json::json;
use serde_json::Value;

use crate::RateLimitGuard;

#[post("/uptime", data = "<data>")]
pub async fn v1_post_uptime(data: Json<UptimeQuery<'_>>, _ratelimit: RateLimiter<'_, RateLimitGuard>) -> (Status, Value) {
    let username = env::var("API_SCYLLADB_USERNAME");
    let passwd = env::var("API_SCYLLADB_PASSWORD");
    if username.is_err() || passwd.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let session = SessionBuilder::new()
        .known_node("localhost:9042")
        .compression(Some(Compression::Lz4))
        .user(username.unwrap(), passwd.unwrap())
        .build()
        .await;
    if session.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let session = session.unwrap();
    let statement = session.prepare("SELECT current_time FROM main.start_timestamp WHERE bot_name = ?").await;
    if statement.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let statement = statement.unwrap();
    let result = session.execute(&statement, (data.0.component_name().to_string(),)).await;
    if result.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let result = result.unwrap();
    let rows = result.rows.unwrap();
    if rows.is_empty() {
        return (Status::NotFound, StatusFns::not_found());
    }

    let row = rows.get(0).unwrap();
    let value = row.columns.get(0).unwrap().as_ref().unwrap();
    let duration = Duration::from_cql(value.clone());
    if duration.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let millis = Utc::now().timestamp_millis() - duration.unwrap().num_milliseconds();

    (Status::Ok, json!({
        "code": 200,
        "message": "ok",
        "data": {
            "elapsed_millis": millis,
        }
    }))
}
