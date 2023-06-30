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

use hartex_backend_models_v1::uptime::UptimeQuery;
use hartex_backend_models_v1::uptime::UptimeResponse;
use hartex_backend_ratelimiter::RateLimiter;
use hartex_backend_status_util::StatusFns;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use serde_json::Value;
use sqlx::postgres::PgConnection;
use sqlx::postgres::PgTypeInfo;
use sqlx::prelude::Connection;
use sqlx::prelude::Executor;
use sqlx::prelude::Statement;
use sqlx::Error;

use crate::RateLimitGuard;

/// # `POST /uptime`
///
/// Obtain the uptime of a certain component.
#[post("/uptime", data = "<data>")]
pub async fn v1_post_uptime(
    data: Json<UptimeQuery<'_>>,
    _ratelimit: RateLimiter<'_, RateLimitGuard>,
) -> (Status, Value) {
    let connect_res = PgConnection::connect(&env::var("API_PGSQL_URL").unwrap()).await;
    if connect_res.is_err() {
        return (
            Status::InternalServerError,
            StatusFns::internal_server_error(),
        );
    }

    let connection = connect_res.unwrap();
    let statement_res = connection
        .prepare_with(
            r#"SELECT * FROM public."StartTimestamps" WHERE component = $1;"#,
            &[PgTypeInfo::with_name("TEXT")],
        )
        .await;
    if statement_res.is_err() {
        return (
            Status::InternalServerError,
            StatusFns::internal_server_error(),
        );
    }

    let statement = statement_res.unwrap();
    let response_res = statement
        .query_as::<UptimeResponse>()
        .bind(data.0.component_name())
        .fetch_one(&connection)
        .await;

    if let Err(&Error::RowNotFound) = &response_res {
        return (
            Status::NotFound,
            StatusFns::not_found(),
        );
    }

    todo!()
}
