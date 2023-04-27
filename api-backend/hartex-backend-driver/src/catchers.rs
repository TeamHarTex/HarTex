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

//! # Error Catchers
//!
//! This module defines certain error catchers for the API backend to return JSON payloads.
//!
//! These catchers are invoked by the main backend process when a certain error has occurred and
//! the status code of that error has an error catcher registered.

use rocket::http::Status;
use rocket::catch;
use rocket::Request;
use serde_json::Value;
use hartex_backend_ratelimiter::error::LimitError;
use hartex_backend_status_util::StatusFns;

/// # 404 Catcher
///
/// This catcher catches the Not Found error.
///
/// An example of the JSON payload:
/// ```json
/// {
///     "code": 404,
///     "message": "the requested resource does not exist"
/// }
/// ```
#[catch(404)]
pub fn not_found(_: &Request) -> (Status, Value) {
    (Status::NotFound, StatusFns::not_found())
}

/// # 429 Catcher
///
/// This catcher catches the Too Many Requests error.
///
/// An example of the JSON payload:
/// ```json
/// {
///     "code": 429,
///     "message": "too many requests"
/// }
/// ```
#[catch(429)]
pub fn too_many_requests<'r>(request: &'r Request) -> &'r LimitError {
    let result: &Result<(), LimitError> = request.local_cache(|| Err(LimitError::UnknownError));
    if let Err(error) = result {
        error
    } else {
        &LimitError::UnknownError
    }
}
