/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

use axum::http::StatusCode;
use axum::Json;
use either::Either;
use serde::Deserialize;
use serde::Serialize;

pub mod uptime;

/// An API response object.
///
/// This is the object returned by a certain API endpoint.
#[derive(Deserialize, Serialize)]
pub struct Response<T, R> {
    pub code: u16,
    message: String,
    data: Either<Option<T>, R>,
}

impl<'a, T, R> Response<T, R>
where
    T: Clone + Deserialize<'a>,
    R: Clone + Deserialize<'a>,
{
    /// Constructs a response object from a status code and data.
    #[allow(clippy::missing_panics_doc)]
    pub fn from_code_with_data(
        code: StatusCode,
        data: Either<Option<T>, R>,
    ) -> (StatusCode, Json<Response<T, R>>) {
        let code_display = code.to_string();
        let part = code_display.split_once(' ').unwrap().1;

        (
            code,
            Json(Self {
                code: code.as_u16(),
                message: part.to_lowercase(),
                data,
            }),
        )
    }

    /// Constructs a response object with a status code of 200 and its corresponding message.
    pub fn ok(value: T) -> (StatusCode, Json<Response<T, R>>) {
        Self::from_code_with_data(StatusCode::OK, Either::Left(Some(value)))
    }

    /// Constructs a response object with a status code of 500 and its corresponding message.
    pub fn internal_server_error() -> (StatusCode, Json<Response<T, R>>) {
        Self::from_code_with_data(StatusCode::INTERNAL_SERVER_ERROR, Either::Left(None))
    }
}

impl<T> Response<T, String> {
    /// Constructs a response object with a status code of 404 and its corresponding message.
    pub fn not_found(component_missing: String) -> (StatusCode, Json<Response<T, String>>) {
        Self::from_code_with_data(
            StatusCode::NOT_FOUND,
            Either::Right(format!("{component_missing} not found")),
        )
    }
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
    pub fn data(&self) -> Option<T> {
        self.data.clone()
    }
}
