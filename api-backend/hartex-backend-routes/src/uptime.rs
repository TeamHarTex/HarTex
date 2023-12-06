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

use std::env;

use axum::Json;
use hartex_backend_models::APIVersion;
use hartex_backend_models::Response;
use hartex_backend_models_v2::uptime::UptimeQuery;
use hartex_backend_models_v2::uptime::UptimeResponse;
use hartex_database_queries::api_backend::queries::start_timestamp_select_by_component::select_start_timestamp_by_component;
use tokio_postgres::NoTls;

/// # `POST /stats/uptime`
///
/// Obtain the uptime of a certain component.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]  // this function cannot panic
#[allow(clippy::module_name_repetitions)]
pub async fn post_uptime(
    _: APIVersion,
    Json(query): Json<UptimeQuery<'_>>,
) -> Json<Response<UptimeResponse>> {
    let result = env::var("API_PGSQL_URL");
    if result.is_err() {
        return Response::internal_server_error();
    }

    let result = tokio_postgres::connect(&result.unwrap(), NoTls).await;
    if result.is_err() {
        return Response::internal_server_error();
    }

    let (client, _) = result.unwrap();
    let result = select_start_timestamp_by_component()
        .bind(&client, &query.component_name())
        .one()
        .await;

    // FIXME: figure out whether the data is actually not found and return 404
    if result.is_err() {
        return Response::internal_server_error();
    }
    let data = result.unwrap();

    Response::ok(UptimeResponse::with_start_timestamp(data.timestamp.unix_timestamp() as u128))
}
