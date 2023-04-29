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

/// # Backend API Status Functions
///
/// Functions for returning JSON payloads for error statuses.

use serde_json::json;
use serde_json::Value;

/// Status functions.
pub struct StatusFns;

impl StatusFns {
    /// Returns a 404 JSON payload.
    pub fn not_found() -> Value {
        json!({
            "code": 404,
            "message": "the requested resource does not exist"
        })
    }

    /// Returns a 405 JSON payload.
    pub fn method_not_allowed() -> Value {
        json!({
            "code": 405,
            "message": "method not allowed"
        })
    }

    /// Returns a 429 JSON payload.
    pub fn too_many_requests() -> Value {
        json!({
            "code": 429,
            "message": "too many requests"
        })
    }

    /// Returns a 500 JSON payload.
    pub fn internal_server_error() -> Value {
        json!({
            "code": 500,
            "message": "internal server error"
        })
    }
}
