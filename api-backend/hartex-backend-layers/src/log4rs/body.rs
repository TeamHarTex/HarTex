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
use std::task::ready;
use std::task::Poll;
use std::time::Instant;

use http_body::Body;
use http_body::Frame;
use http_body::SizeHint;
use log::Metadata;
use pin_project::pin_project;
use tower_http::classify::ClassifyEos;
use crate::log4rs::on_body_chunk::OnBodyChunk;

#[pin_project]
pub struct Log4rsResponseBody<'a, B, C, OnBodyChunkT> {
    #[pin]
    pub(crate) inner: B,
    pub(crate) classify_eos: C,
    pub(crate) on_body_chunk: OnBodyChunkT,
    pub(crate) start: Instant,
    pub(crate) metadata: Metadata<'a>
}

impl<B, C, OnBodyChunkT> Body for Log4rsResponseBody<B, C, OnBodyChunkT>
where
    B: Body,
    B::Error: Display + 'static,
    C: ClassifyEos,
    OnBodyChunkT: OnBodyChunk<B::Data>,
{
    type Data = B::Data;
    type Error = B::Error;

    fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let projected = self.project();
        let result = ready!(projected.inner.poll_frame(cx));

        let latency = projected.start.elapsed();
        *projected.start = Instant::now();

        match result {
            Some(Ok(frame)) => {
                let frame = match frame.into_data() {
                    Ok(chunk) => {
                        projected.on_body_chunk.on_body_chunk(&chunk, latency, &projected.metadata);
                        Frame::data(chunk)
                    }
                    Err(frame) => frame
                };

                Poll::Ready(Some(Ok(frame)))
            }
            _ => Poll::Pending,
        }
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}
