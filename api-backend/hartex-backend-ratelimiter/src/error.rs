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

/// # Ratelimit Errors
///
/// Ratelimit errors that may be returned by the ratelimiter.

use std::io::Cursor;
use governor::Quota;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use rocket::Response;
use rocket::response::Responder;
use rocket::response::Result;
use hartex_backend_status_util::StatusFns;

/// A ratelimit error.
#[derive(Clone, Debug)]
pub enum LimitError {
    /// The Client IP of the request is not specified.
    ClientIpNotSpecified,
    /// The request is ratelimited.
    RequestRateLimited(u128, Quota),
    /// The route name of the request is not specified.
    RouteNameNotSpecified,
    /// The route of the request is not specified.
    RouteNotSpecified,
    /// An unknown error has occured.
    UnknownError,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for &LimitError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'o> {
        let mut response = Response::build()
            .header(ContentType::JSON)
            .finalize();

        match self {
            LimitError::ClientIpNotSpecified => {
                response.set_status(Status::InternalServerError);

                let body = StatusFns::internal_server_error().to_string();
                response.set_sized_body(body.len(), Cursor::new(body));
            }
            LimitError::RequestRateLimited(wait_time, quota) => {
                response.set_status(Status::TooManyRequests);

                response.set_raw_header("Retry-After", wait_time.to_string());
                response.set_raw_header("X-RateLimit-Limit", quota.burst_size().to_string());
                response.set_raw_header("X-RateLimit-Reset-After", quota.burst_size_replenished_in().as_secs().to_string());

                let body = StatusFns::too_many_requests().to_string();
                response.set_sized_body(body.len(), Cursor::new(body));
            }
            LimitError::RouteNameNotSpecified => {
                response.set_status(Status::InternalServerError);

                let body = StatusFns::internal_server_error().to_string();
                response.set_sized_body(body.len(), Cursor::new(body));
            }
            LimitError::RouteNotSpecified => {
                response.set_status(Status::InternalServerError);

                let body = StatusFns::internal_server_error().to_string();
                response.set_sized_body(body.len(), Cursor::new(body));
            }
            LimitError::UnknownError => {
                response.set_status(Status::InternalServerError);

                let body = StatusFns::internal_server_error().to_string();
                response.set_sized_body(body.len(), Cursor::new(body));
            }
        }

        Ok(response)
    }
}
