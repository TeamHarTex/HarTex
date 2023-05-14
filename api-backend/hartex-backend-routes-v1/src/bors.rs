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

use rocket::get;
use rocket::http::Status;
use serde_json::Value;

/// # Bors Routes
///
/// Routes interacting with the bors API.

/// # `GET /bors/repository/<repository>/permissions/<permission>`
///
/// Obtain the list of users having the specified permission in a repository.
#[get("/bors/repository/<repository>/permissions/<permission>")]
pub async fn v1_get_bors_user_list_with_permissions_in_repository(
    repository: String,
    permission: String,
) -> (Status, Value) {
    todo!()
}
