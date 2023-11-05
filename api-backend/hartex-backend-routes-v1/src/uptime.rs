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

use hartex_backend_models_v1::uptime::UptimeQuery;
use hartex_backend_ratelimiter::RateLimiter;
use rocket::post;
use rocket::response::Redirect;
use rocket::serde::json::Json;

use crate::RateLimitGuard;

/// # `POST /uptime`
///
/// Obtain the uptime of a certain component.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]  // this function cannot panic
#[allow(clippy::module_name_repetitions)]
#[post("/uptime", data = "<data>")]
pub async fn v1_post_uptime(
    data: Json<UptimeQuery<'_>>,
    _ratelimit: RateLimiter<'_, RateLimitGuard>,
) -> Redirect {
    Redirect::moved("/api/v2/uptime")
}
