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

use governor::Quota;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use rocket::Response;
use rocket::response::Responder;

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

        // todo: finish this
        match self {
            LimitError::ClientIpNotSpecified => {
                response.set_status(Status::InternalServerError);
            }
            LimitError::RequestRateLimited(_, _) => {
                response.set_status(Status::TooManyRequests);
            }
            LimitError::RouteNameNotSpecified => {
                response.set_status(Status::InternalServerError);
            }
            LimitError::RouteNotSpecified => {
                response.set_status(Status::InternalServerError);
            }
        }

        Ok(response)
    }
}
