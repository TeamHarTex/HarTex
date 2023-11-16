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

//! # Bors Routes
//!
//! Routes interacting with the bors API.

use std::env;

use hartex_backend_ratelimiter::RateLimiter;
use hartex_backend_status_util::StatusFns;
use hartex_database_queries::api_backend::queries::bors_repository_permissions_select_user_by_repository_and_permissions::select_user_by_repository_and_permissions;
use hartex_database_queries::api_backend::types::public::BorsRepositoryPermissions;
use hartex_log::log;
use rocket::get;
use rocket::http::Status;
use serde_json::json;
use serde_json::Value;
use tokio_postgres::NoTls;

use crate::RateLimitGuard;

/// # `GET /bors/repositories/<repository>/permissions/<permission>`
///
/// Obtain the list of users having the specified permission in a repository.
#[allow(clippy::missing_panics_doc)]  // this function cannot panic
#[allow(clippy::unused_async)]
#[allow(unused_variables)]
#[get("/bors/repositories/<repository>/permissions/<permission>")]
pub async fn v2_repositories_repository_permissions_permissions(
    repository: String,
    permission: String,
    _ratelimit: RateLimiter<'_, RateLimitGuard>,
) -> (Status, Value) {
    log::trace!("attempting to retrieve permissions data");

    let result = env::var("API_PGSQL_URL");
    if result.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let result = tokio_postgres::connect(&result.unwrap(), NoTls).await;
    if result.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    let (client, _) = result.unwrap();
    let permission_enum = match &*permission {
        "review" => BorsRepositoryPermissions::review,
        "try" => BorsRepositoryPermissions::r#try,
        _ => return (Status::NotFound, StatusFns::not_found()),
    };
    let result = select_user_by_repository_and_permissions()
        .bind(&client, &repository, vec![permission_enum])
        .all()
        .await;
    if result.is_err() {
        return (Status::InternalServerError, StatusFns::internal_server_error());
    }

    (
        Status::Ok,
        json!({
            "code": 200,
            "message": "ok",
            "data": {
                "users": result.unwrap(),
            }
        }),
    )
}
