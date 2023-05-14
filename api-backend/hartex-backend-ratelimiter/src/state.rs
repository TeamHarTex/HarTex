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

/// # Ratelimited Request State
use governor::Quota;

/// A cached state for requests coming into the API backend.
#[derive(Debug)]
pub struct RequestState {
    pub(crate) quota: Quota,
    pub(crate) request_capacity: u32,
}

impl RequestState {
    pub(crate) fn new(quota: Quota, request_capacity: u32) -> Self {
        Self {
            quota,
            request_capacity,
        }
    }

    /// The remaining quota.
    pub fn quota(&self) -> &Quota {
        &self.quota
    }

    /// The request capacity.
    pub fn request_capacity(&self) -> u32 {
        self.request_capacity
    }
}
