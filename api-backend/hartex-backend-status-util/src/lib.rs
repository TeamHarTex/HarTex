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

use serde_json::json;
use serde_json::Value;

pub struct StatusFns;

impl StatusFns {
    pub fn not_found() -> Value {
        json!({
            "code": 404,
            "message": "the requested resource does not exist"
        })
    }

    pub fn too_many_requests() -> Value {
        json!({
            "code": 429,
            "message": "too many requests"
        })
    }

    pub fn internal_server_error() -> Value {
        json!({
            "code": 500,
            "message": "internal server error"
        })
    }
}
