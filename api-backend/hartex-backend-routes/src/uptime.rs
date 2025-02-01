/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # Uptime Routes
//!
//! Routes interacting with the uptime API.

use axum::Json;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::WithRejection;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::GenericClient;
use bb8_postgres::tokio_postgres::NoTls;
use chrono::DateTime;
use futures_util::stream::TryStreamExt;
use hartex_backend_models::Response;
use hartex_backend_models::uptime::UptimeQuery;
use hartex_backend_models::uptime::UptimeQueryRejection;
use hartex_backend_models::uptime::UptimeResponse;
use hartex_backend_models::uptime::UptimeUpdate;
use hartex_database_queries::queries::api_backend::start_timestamp_select_by_component::StartTimestampSelectByComponent;
use hartex_database_queries::queries::api_backend::start_timestamp_upsert::StartTimestampUpsert;
use hartex_log::log;

/// Get component uptime
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)] // this function cannot panic
#[allow(clippy::module_name_repetitions)]
#[utoipa::path(
    get,
    path = "/api/v1/stats/uptime",
    params(UptimeQuery),
    responses(
        (status = 200, description = "Uptime retrieved successfully", body = UptimeResponse),
        (status = 404, description = "Specified component not found in uptime database"),
        (status = 422, description = "Bad request body"),
        (status = 500, description = "Generic internal server error")
    )
)]
pub async fn get_uptime(
    WithRejection(Query(query), _): WithRejection<Query<UptimeQuery>, UptimeQueryRejection>,
) -> (StatusCode, Json<Response<UptimeResponse, String>>) {
    log::trace!("querying timestamp");
    let name = query.component_name();
    let result = StartTimestampSelectByComponent::bind(name.to_string())
        .executor()
        .await;

    if result.is_err() {
        return Response::internal_server_error();
    }

    let result = result.unwrap().many().await;

    if result.is_err() {
        return Response::internal_server_error();
    }

    let vec = result.unwrap();
    if vec.is_empty() {
        return Response::not_found(String::from("component"));
    }

    Response::ok(UptimeResponse::with_start_timestamp(
        vec[0].timestamp().timestamp() as u128,
    ))
}

/// Update component uptime
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)] // this function cannot panic
#[allow(clippy::module_name_repetitions)]
#[utoipa::path(
    patch,
    path = "/api/v1/stats/uptime",
    request_body(content = UptimeUpdate, description = "The uptime update payload for a component"),
    responses(
        (status = 204, description = "Uptime updated successfully"),
        (status = 400, description = "Unable to process request body"),
        (status = 500, description = "Generic internal server error")
    )
)]
pub async fn patch_uptime(
    Json(query): Json<UptimeUpdate>,
) -> (StatusCode, Json<Response<(), String>>) {
    log::trace!("updating timestamp");

    let Some(timestamp) = DateTime::from_timestamp(query.start_timestamp() as i64, 0) else {
        // FIXME: return a better status code as the timestamp is out of range if this branch is reached
        // just 500 for now
        return Response::internal_server_error();
    };
    let result = StartTimestampUpsert::bind(query.component_name().to_string(), timestamp)
        .executor()
        .await;

    if result.is_err() {
        return Response::internal_server_error();
    }

    let result = result.unwrap().execute().await;

    if result.is_err() {
        return Response::internal_server_error();
    }

    Response::no_content()
}
