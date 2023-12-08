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
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use http_body::Body;
use http_body::Frame;
use http_body::SizeHint;
use pin_project_lite::pin_project;
use tower_http::classify::ClassifyEos;

pin_project! {
    pub struct Log4rsResponseBody<B, C> {
        #[pin]
        pub(crate) inner: B,
        pub(crate) classify_eos: C,
    }
}

impl<B, C> Body for Log4rsResponseBody<B, C>
where
    B: Body,
    B::Error: Display + 'static,
    C: ClassifyEos,
{
    type Data = ();
    type Error = ();

    fn poll_frame(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        todo!()
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}
