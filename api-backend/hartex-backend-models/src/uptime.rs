/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # Uptime Models V2
//!
//! Models for the uptime API specification V2 of the backend.

use axum::extract::rejection::QueryRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use either::Either;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

/// An uptime query.
#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, IntoParams, Serialize)]
#[into_params(parameter_in = Query)]
pub struct UptimeQuery {
    component: String,
}

impl UptimeQuery {
    /// Create a new uptime query with the component name to search for.
    #[must_use]
    pub fn new(component: &str) -> Self {
        Self {
            component: component.to_string(),
        }
    }

    /// The component name to search for in this uptime query.
    #[must_use]
    pub fn component_name(&self) -> &str {
        self.component.as_str()
    }
}

pub struct UptimeQueryRejection {
    status_code: StatusCode,
    data_message: String,
}

impl From<QueryRejection> for UptimeQueryRejection {
    fn from(value: QueryRejection) -> Self {
        Self {
            status_code: value.status(),
            data_message: value.body_text().to_lowercase(),
        }
    }
}

impl IntoResponse for UptimeQueryRejection {
    fn into_response(self) -> Response {
        crate::Response::<String, ()>::from_code_with_data(
            self.status_code,
            Either::Left(Some(self.data_message)),
        )
        .into_response()
    }
}

/// The uptime of the specified component.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct UptimeResponse {
    start_timestamp: u128,
}

impl UptimeResponse {
    /// Constructs a response to an uptime query with the start timestamp.
    #[must_use]
    pub fn with_start_timestamp(start_timestamp: u128) -> Self {
        Self { start_timestamp }
    }

    /// The start timestamp of the uptime data.
    #[must_use]
    pub fn start_timestamp(&self) -> u128 {
        self.start_timestamp
    }
}

/// An uptime update payload.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct UptimeUpdate {
    component_name: String,
    start_timestamp: u128,
}

impl UptimeUpdate {
    /// Constructs a new uptime update payload.
    #[must_use]
    pub fn new(component_name: impl Into<String>, start_timestamp: u128) -> Self {
        Self {
            component_name: component_name.into(),
            start_timestamp,
        }
    }

    /// The component name of the uptime update.
    #[must_use]
    pub fn component_name(&self) -> &str {
        self.component_name.as_str()
    }

    /// The start timestamp of the uptime update.
    #[must_use]
    pub fn start_timestamp(&self) -> u128 {
        self.start_timestamp
    }
}
