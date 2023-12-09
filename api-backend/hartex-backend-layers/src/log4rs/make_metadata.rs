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

use http::Request;
use log::Level;
use log::Metadata;

pub trait MakeMetadata<B> {
    fn make_metadata(&mut self, request: &Request<B>) -> Metadata;
}

#[derive(Clone, Debug)]
pub struct DefaultMakeMetadata {
    level: Level,
    headers: bool,
}

impl DefaultMakeMetadata {
    pub fn new() -> Self {
        Self {
            level: Level::Trace,
            headers: false,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn headers(mut self, headers: bool) -> Self {
        self.headers = headers;
        self
    }
}

impl Default for DefaultMakeMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> MakeMetadata<B> for DefaultMakeMetadata {
    fn make_metadata(&mut self, _: &Request<B>) -> Metadata {
        Metadata::builder()
            .level(self.level)
            .target("request")
            .build()
    }
}
