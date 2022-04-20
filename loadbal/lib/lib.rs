/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! Library interface of the load balancer.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    target_server_type: String,
    method: String,
    route: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn target_server_type(&self) -> &str {
        self.target_server_type.as_str()
    }

    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn route(&self) -> &str {
        self.route.as_str()
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}
