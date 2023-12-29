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

//! # Uptime Models V1
//!
//! Models for the uptime API specification V1 of the backend.

use serde::Deserialize;
use serde::Serialize;

/// An uptime query.
#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, Serialize)]
pub struct UptimeQuery {
    component_name: String,
}

impl UptimeQuery {
    /// Create a new uptime query with the component name to search for.
    #[must_use]
    pub fn new(component_name: &str) -> Self {
        Self {
            component_name: component_name.to_string(),
        }
    }

    /// The component name to search for in this uptime query.
    #[must_use]
    pub fn component_name(&self) -> &str {
        self.component_name.as_str()
    }
}

/// A response to an uptime query.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Deserialize, Serialize)]
pub struct UptimeResponse {
    start_timestamp: u128,
}

impl UptimeResponse {
    #[must_use]
    pub fn with_start_timestamp(start_timestamp: u128) -> Self {
        Self { start_timestamp }
    }

    /// The start timestamp of the uptime entry.
    #[must_use]
    pub fn start_timestamp(&self) -> u128 {
        self.start_timestamp
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Deserialize, Serialize)]
pub struct UptimeUpdate {
    component_name: String,
    start_timestamp: u128,
}

impl UptimeUpdate {
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
