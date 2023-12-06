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

//! # Backend General Models
//!
//! The general models crate provides general models regarding the API backend.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::collections::HashMap;

use axum::async_trait;
use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::extract::Path;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response as AxumResponse;
use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
pub enum APIVersion {
    V1,
    V2,
}

#[async_trait]
impl<S> FromRequestParts<S> for APIVersion
where
    S: Send + Sync,
{
    type Rejection = AxumResponse;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let parameters: Path<HashMap<String, String>> = parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = parameters
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version not specified").into_response())?;

        match version.as_str() {
            "v1" => Ok(APIVersion::V1),
            "v2" => Ok(APIVersion::V2),
            _ => Err((StatusCode::NOT_FOUND, "unknown version specified").into_response()),
        }
    }
}

/// An API response object.
///
/// This is the object returned by a certain API endpoint.
#[derive(Deserialize)]
pub struct Response<T> {
    code: u16,
    message: String,
    data: T,
}

impl<'a, T> Response<T>
where
    T: Clone + Deserialize<'a>,
{
    /// The status code of the response.
    pub fn code(&self) -> u16 {
        self.code
    }

    /// The message of the response.
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// The data of the response.
    pub fn data(&self) -> T {
        self.data.clone()
    }
}
