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

use std::io::Cursor;
use governor::Quota;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use rocket::Response;
use rocket::response::Responder;
use hartex_backend_status_util::StatusFns;
use crate::state::RequestState;

#[derive(Clone, Debug)]
pub enum LimitError {
    ClientIpNotSpecified,
    RequestRateLimited(u128, Quota),
    RouteNameNotSpecified,
    RouteNotSpecified,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for &LimitError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
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

                let state = request.local_cache(|| RequestState::new(quota.clone(), 0));

                response.set_raw_header("Retry-After", wait_time.to_string());
                response.set_raw_header("X-RateLimit-Limit", quota.burst_size().to_string());
                response.set_raw_header("X-RateLimit-Remaining", state.request_capacity.to_string());
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
        }

        Ok(response)
    }
}
