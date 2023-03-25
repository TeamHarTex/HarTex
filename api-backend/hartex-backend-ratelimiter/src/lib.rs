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
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

use crate::limitable::Limitable;

pub mod limitable;
pub(crate) mod registry;

pub struct RateLimiter<'r, L>
where
    L: Limitable<'r>, {
    phantom: PhantomData<&'r L>,
}

impl<'r, L> RateLimiter<'r, L>
where
    L: Limitable<'r>, {
    pub fn handle_from_request(_: &'r Request) -> Outcome<Self, ()> {
        todo!()
    }
}

#[async_trait]
impl<'r, L> FromRequest<'r> for RateLimiter<'r, L>
where
    L: Limitable<'r>, {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Self::handle_from_request(request)
    }
}
