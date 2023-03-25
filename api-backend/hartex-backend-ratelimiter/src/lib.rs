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

use std::marker::PhantomData;

use async_trait::async_trait;
use governor::clock::Clock;
use governor::clock::DefaultClock;
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

use crate::error::LimitError;
use crate::limitable::Limitable;
use crate::registry::Registry;
use crate::state::RequestState;

pub mod error;
pub mod limitable;
pub(crate) mod registry;
pub mod state;

pub struct RateLimiter<'r, L>
where
    L: Limitable<'r>, {
    phantom: PhantomData<&'r L>,
}

impl<'r, L> RateLimiter<'r, L>
where
    L: Limitable<'r>, {
    pub fn handle_from_request(request: &'r Request) -> Outcome<Self, LimitError> {
        let result = request.local_cache(|| {
            if let Some(route) = request.route() {
                if let Some(route_name) = &route.name {
                    let limiter = Registry::get_or_insert::<L>(
                        route.method,
                        route_name,
                        L::evaluate_limit(route.method, route_name)
                    );

                    if let Some(client_ip) = request.client_ip() {
                        let limit_check_result = limiter.check_key(&client_ip);

                        match limit_check_result {
                            Ok(state) => {
                                let request_capacity = state.remaining_burst_capacity();
                                let request_state = RequestState::new(state.quota(), request_capacity);

                                let _ = request.local_cache(|| request_state);

                                Ok(())
                            }
                            Err(not_until) => {
                                let wait_time = not_until.wait_time_from(CLOCK.now()).as_millis();
                                Err(LimitError::RequestRateLimited(wait_time, not_until.quota()))
                            }
                        }
                    } else {
                        Err(LimitError::ClientIpNotSpecified)
                    }
                } else {
                    Err(LimitError::RouteNameNotSpecified)
                }
            } else {
                Err(LimitError::RouteNotSpecified)
            }
        });

        match result {
            Ok(_) => Outcome::Success(Self::default()),
            Err(error) => {
                let error = error.clone();

                match error {
                    LimitError::RequestRateLimited(_, _) => {
                        Outcome::Failure((Status::TooManyRequests, error))
                    }
                    _ => Outcome::Failure((Status::BadRequest, error))
                }
            }
        }
    }
}

impl<'r, T> Default for RateLimiter<'r, T>
    where
        T: Limitable<'r>,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<'r, L> FromRequest<'r> for RateLimiter<'r, L>
where
    L: Limitable<'r>, {
    type Error = LimitError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Self::handle_from_request(request)
    }
}

lazy_static! {
    static ref CLOCK: DefaultClock = DefaultClock::default();
}
