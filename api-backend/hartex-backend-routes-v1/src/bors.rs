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

use std::fs::File;
use std::io::Read;

use hartex_backend_status_util::StatusFns;
use rocket::get;
use rocket::http::Status;
use serde_json::json;
use serde_json::Value;

/// # `GET /bors/repository/<repository>/permissions/<permission>`
///
/// Obtain the list of users having the specified permission in a repository.
#[get("/bors/repository/<repository>/permissions/<permission>")]
pub async fn v1_repository_repository_permissions_permissions(
    repository: String,
    permission: String,
) -> (Status, Value) {
    let result = File::open(format!(
        "../backend-data/bors.{}.permissions.{}",
        repository.to_lowercase(),
        permission.to_lowercase()
    ));

    if result.is_err() {
        return (Status::NotFound, StatusFns::not_found());
    }

    let mut file = result.unwrap();
    let mut buffer = String::new();
    if let Err(_) = file.read_to_string(&mut buffer) {
        return (
            Status::InternalServerError,
            StatusFns::internal_server_error(),
        );
    }

    let users = buffer.lines().collect::<Vec<_>>();

    (
        Status::Ok,
        json!({
            "code": 200,
            "message": "ok",
            "data": {
                "github_users": users
            }
        }),
    )
}
