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

use serde::Deserialize;

/// An API response object.
///
/// This is the object returned by a certain API endpoint.
#[derive(Deserialize)]
pub struct Response<'a, T> {
    code: u16,
    message: &'a str,
    data: T,
}

impl<'a, T> Response<'a, T>
where
    T: Clone + Deserialize<'a>,
{
    /// The status code of the response.
    pub fn code(&self) -> u16 {
        self.code
    }

    /// The message of the response.
    pub fn message(&self) -> &'a str {
        self.message
    }

    /// The data of the reponse.
    pub fn data(&self) -> T {
        self.data.clone()
    }
}
