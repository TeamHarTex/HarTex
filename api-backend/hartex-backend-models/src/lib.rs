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
use serde::Deserialize;
use serde::Serialize;

pub mod uptime;

/// An API response object.
///
/// This is the object returned by a certain API endpoint.
#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub code: u16,
    message: String,
    data: Option<T>,
}

impl<'a, T> Response<T>
where
    T: Clone + Deserialize<'a>,
{
    pub fn from_code_with_data(code: StatusCode, data: T) -> (StatusCode, Json<Response<T>>) {
        let code_display = code.to_string();
        let part = code_display.split_once(" ").unwrap().1;

        (
            code,
            Json(Self {
                code: code.as_u16(),
                message: part.to_lowercase(),
                data: Some(data),
            }),
        )
    }

    /// Constructs a response object with a status code of 500 and its corresponding message.
    pub fn internal_server_error() -> (StatusCode, Json<Response<T>>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Self {
                code: 500,
                message: String::from("internal server error"),
                data: None,
            }),
        )
    }

    /// Constructs a response object with a status code of 200 and its corresponding message.
    pub fn ok(value: T) -> (StatusCode, Json<Response<T>>) {
        (
            StatusCode::OK,
            Json(Self {
                code: 200,
                message: String::from("ok"),
                data: Some(value),
            }),
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
