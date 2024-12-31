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

use axum::Json;
/// # Uptime Routes
///
/// Routes interacting with the uptime API.
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::WithRejection;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::GenericClient;
use bb8_postgres::tokio_postgres::NoTls;
use futures_util::stream::TryStreamExt;
use hartex_backend_models::Response;
use hartex_backend_models::uptime::UptimeQuery;
use hartex_backend_models::uptime::UptimeQueryRejection;
use hartex_backend_models::uptime::UptimeResponse;
use hartex_backend_models::uptime::UptimeUpdate;
use hartex_database_queries::api_backend::queries::start_timestamp_select_by_component::select_start_timestamp_by_component;
use hartex_database_queries::api_backend::queries::start_timestamp_upsert::start_timestamp_upsert;
use hartex_log::log;
use time::OffsetDateTime;

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
    State(pool): State<Pool<PostgresConnectionManager<NoTls>>>,
    WithRejection(Query(query), _): WithRejection<Query<UptimeQuery>, UptimeQueryRejection>,
) -> (StatusCode, Json<Response<UptimeResponse, String>>) {
    log::trace!("retrieving connection from database pool");
    let result = pool.get().await;
    if result.is_err() {
        return Response::internal_server_error();
    }

    let connection = result.unwrap();
    let client = connection.client();

    log::trace!("querying timestamp");
    let name = query.component_name();
    let mut query = select_start_timestamp_by_component();
    let result = query.bind(client, &name).iter().await;

    if result.is_err() {
        return Response::internal_server_error();
    }

    let iterator = result.unwrap();
    let result = iterator.try_collect::<Vec<_>>().await;
    if result.is_err() {
        log::error!("{:?}", result.unwrap_err());

        return Response::internal_server_error();
    }

    let vec = result.unwrap();
    if vec.is_empty() {
        return Response::not_found(String::from("component"));
    }

    Response::ok(UptimeResponse::with_start_timestamp(
        vec[0].timestamp.unix_timestamp() as u128,
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
    State(pool): State<Pool<PostgresConnectionManager<NoTls>>>,
    Json(query): Json<UptimeUpdate>,
) -> (StatusCode, Json<Response<(), String>>) {
    log::trace!("retrieving connection from database pool");
    let result = pool.get().await;
    if result.is_err() {
        return Response::internal_server_error();
    }

    let connection = result.unwrap();
    let client = connection.client();

    log::trace!("updating timestamp");
    let Ok(timestamp) = OffsetDateTime::from_unix_timestamp(query.start_timestamp() as i64) else {
        // FIXME: return a better status code as the timestamp is out of range if this branch is reached
        // just 500 for now
        return Response::internal_server_error();
    };
    let result = start_timestamp_upsert()
        .bind(client, &query.component_name(), &timestamp)
        .await;

    if result.is_err() {
        return Response::internal_server_error();
    }

    Response::no_content()
}
