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

/// # Uptime Routes
///
/// Routes interacting with the uptime API.

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::GenericClient;
use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
use hartex_backend_models::APIVersion;
use hartex_backend_models::Response;
use hartex_backend_models_v2::uptime::UptimeQuery;
use hartex_backend_models_v2::uptime::UptimeResponse;
use hartex_backend_models_v2::uptime::UptimeUpdate;
use hartex_database_queries::api_backend::queries::start_timestamp_select_by_component::select_start_timestamp_by_component;
use hartex_database_queries::api_backend::queries::start_timestamp_upsert::start_timestamp_upsert;
use time::OffsetDateTime;

/// # `PATCH /stats/uptime`
///
/// Update the uptime of a certain component.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)] // this function cannot panic
#[allow(clippy::module_name_repetitions)]
pub async fn patch_uptime(
    _: APIVersion,
    State(pool): State<Pool<PostgresConnectionManager<NoTls>>>,
    Json(query): Json<UptimeUpdate>,
) -> (StatusCode, Json<Response<()>>) {
    let result = pool.get().await;
    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::internal_server_error(),
        );
    }

    let connection = result.unwrap();
    let client = connection.client();

    let Ok(timestamp) = OffsetDateTime::from_unix_timestamp(query.start_timestamp() as i64) else {
        // FIXME: return a better status code as the timestamp is out of range if this branch is reached
        // just 500 for now
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::internal_server_error(),
        );
    };
    let result = start_timestamp_upsert()
        .bind(client, &query.component_name(), &timestamp)
        .await;

    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::internal_server_error(),
        );
    }

    (
        StatusCode::OK,
        Response::ok(()),
    )
}

/// # `POST /stats/uptime`
///
/// Obtain the uptime of a certain component.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)] // this function cannot panic
#[allow(clippy::module_name_repetitions)]
pub async fn post_uptime(
    _: APIVersion,
    State(pool): State<Pool<PostgresConnectionManager<NoTls>>>,
    Json(query): Json<UptimeQuery>,
) -> (StatusCode, Json<Response<UptimeResponse>>) {
    let result = pool.get().await;
    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::internal_server_error(),
        );
    }

    let connection = result.unwrap();
    let client = connection.client();

    let result = select_start_timestamp_by_component()
        .bind(client, &query.component_name())
        .one()
        .await;

    // FIXME: figure out whether the data is actually not found and return 404
    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::internal_server_error(),
        );
    }
    let data = result.unwrap();

    (
        StatusCode::OK,
        Response::ok(UptimeResponse::with_start_timestamp(
            data.timestamp.unix_timestamp() as u128,
        )),
    )
}
