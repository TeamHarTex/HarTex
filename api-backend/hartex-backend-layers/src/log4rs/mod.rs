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
use std::task::Context;
use std::task::Poll;
use std::time::Instant;

use http::Request;
use http::Response;
use http_body::Body;
use tower_http::classify::MakeClassifier;
use tower_service::Service;

use crate::log4rs::body::Log4rsResponseBody;
use crate::log4rs::future::Log4rsResponseFuture;
use crate::log4rs::make_metadata::DefaultMakeMetadata;
use crate::log4rs::make_metadata::MakeMetadata;
use crate::log4rs::on_body_chunk::DefaultOnBodyChunk;
use crate::log4rs::on_body_chunk::OnBodyChunk;

pub use layer::Log4rsLayer;

mod body;
mod future;
mod layer;
mod make_metadata;
mod on_body_chunk;

#[derive(Clone, Copy, Debug)]
pub struct Log4rs<'a, S, M, MakeMetadataT = DefaultMakeMetadata, OnBodyChunkT = DefaultOnBodyChunk> {
    pub(crate) inner: S,
    pub(crate) make_classifier: M,
    pub(crate) make_metadata: MakeMetadataT,
    pub(crate) on_body_chunk: OnBodyChunkT,
}

impl<'a, S, M> Log4rs<'a, S, M> {
    pub fn new(inner: S, make_classifier: M) -> Self {
        Self {
            inner,
            make_classifier,
            make_metadata: DefaultMakeMetadata::new(),
            on_body_chunk: DefaultOnBodyChunk::new(),
        }
    }

    pub fn layer(make_classifier: M) -> Log4rsLayer<M> {
        Log4rsLayer::new(make_classifier)
    }
}

impl<'a, S, M, RequestBodyT, ResponseBodyT, MakeMetadataT, OnBodyChunkT> Service<Request<RequestBodyT>> for Log4rs<'a, S, M, MakeMetadataT, OnBodyChunkT>
where
    S: Service<Request<RequestBodyT>, Response = Response<ResponseBodyT>>,
    RequestBodyT: Body,
    ResponseBodyT: Body,
    ResponseBodyT::Error: Display + 'static,
    M: MakeClassifier,
    M::Classifier: Clone,
    MakeMetadataT: MakeMetadata<RequestBodyT>,
    OnBodyChunkT: OnBodyChunk<ResponseBodyT::Data> + Clone,
{
    type Response = Response<Log4rsResponseBody<'a, ResponseBodyT, M::ClassifyEos, OnBodyChunkT>>;
    type Error = S::Error;
    type Future = Log4rsResponseFuture<'a, S::Future, M::Classifier, OnBodyChunkT>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<RequestBodyT>) -> Self::Future {
        let start = Instant::now();

        let metadata = self.make_metadata.make_metadata(&request);
        let classifier = self.make_classifier.make_classifier(&request);

        let future = self.inner.call(request);

        Log4rsResponseFuture {
            inner: future,
            classifier: Some(classifier),
            start,
            on_body_chunk: Some(self.on_body_chunk.clone()),
            metadata,
        }
    }
}
