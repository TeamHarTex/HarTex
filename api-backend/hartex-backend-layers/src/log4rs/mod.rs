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

use http::Request;
use http::Response;
use http_body::Body;
use tower_http::classify::MakeClassifier;
use tower_service::Service;
pub use layer::Log4rsLayer;

use crate::log4rs::make_metadata::DefaultMakeMetadata;

mod body;
mod layer;
mod make_metadata;

#[derive(Clone, Copy, Debug)]
pub struct Log4rs<S, M, MakeMetadata = DefaultMakeMetadata> {
    pub(crate) inner: S,
    pub(crate) make_classifier: M,
    pub(crate) make_metadata: MakeMetadata,
}

impl<S, M> Log4rs<S, M> {
    pub fn new(inner: S, make_classifier: M) -> Self {
        Self {
            inner,
            make_classifier,
            make_metadata: DefaultMakeMetadata::new(),
        }
    }

    pub fn layer(make_classifier: M) -> Log4rsLayer<M> {
        Log4rsLayer::new(make_classifier)
    }
}

impl<S, M, RequestBodyT, ResponseBodyT> Service<Request<RequestBodyT>> for Log4rs<S, M>
where
    S: Service<Request<RequestBodyT>, Response = Response<ResponseBodyT>>,
    RequestBodyT: Body,
    ResponseBodyT: Body,
    ResponseBodyT::Error: Display + 'static,
    M: MakeClassifier,
    M::Classifier: Clone,
{
    type Response = ();
    type Error = S::Error;
    type Future = ();

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, _: Request<RequestBodyT>) -> Self::Future {
        todo!()
    }
}
