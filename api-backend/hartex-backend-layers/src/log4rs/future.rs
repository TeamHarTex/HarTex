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

use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Instant;

use http::Response;
use http_body::Body;
use pin_project_lite::pin_project;
use tower_http::classify::ClassifyResponse;

use crate::log4rs::body::Log4rsResponseBody;

pin_project! {
    pub struct Log4rsResponseFuture<F, C> {
        #[pin]
        pub(crate) inner: F,
        pub(crate) classifier: C,
        pub(crate) start: Instant,
    }
}

impl<FutureT, ResponseBodyT, E, C> Future for Log4rsResponseFuture<FutureT, C>
where
    FutureT: Future<Output = Result<Response<ResponseBodyT>, E>>,
    ResponseBodyT: Body,
    ResponseBodyT::Error: Display + 'static,
    E: Display + 'static,
    C: ClassifyResponse,
{
    type Output = Result<Response<Log4rsResponseBody<ResponseBodyT, C::ClassifyEos>>, E>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
