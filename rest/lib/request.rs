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

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use hyper::{Body, Response, StatusCode};

use serde::{Deserialize, Serialize};
use tokio::time;

#[derive(Clone)]
pub struct Ratelimit {
    limit: i64,
    remaining: i64,
    reset: Option<SystemTime>,
    reset_after: Option<Duration>,
}

impl Ratelimit {
    pub async fn preflight(&mut self) {
        if self.limit == 0 {
            return;
        }

        let Some(reset) = self.reset else {
            self.remaining = self.limit;
            return;
        };

        let Ok(delay) = reset.duration_since(SystemTime::now()) else {
            if self.remaining != 0 {
                self.remaining -= 1;
            }

            return;
        };

        if self.remaining == 0 {
            time::sleep(delay).await;
            return;
        }

        self.remaining -= 1;
    }

    pub async fn postflight(&mut self, response: &Response<Body>) -> bool {
        if let Some(value) = response.headers().get("X-RateLimit-Limit") {
            let str = value.to_str().unwrap();
            let limit = str.parse::<i64>().unwrap();

            self.limit = limit;
        }

        if let Some(value) = response.headers().get("X-RateLimit-Remaining") {
            let str = value.to_str().unwrap();
            let remaining = str.parse::<i64>().unwrap();

            self.remaining = remaining;
        }

        if let Some(value) = response.headers().get("X-RateLimit-Reset-After") {
            let str = value.to_str().unwrap();
            let reset_after = Duration::from_secs_f64(str.parse::<f64>().unwrap());

            self.reset = Some(SystemTime::now() + reset_after);
            self.reset_after = Some(reset_after);
        }

        if response.status() != StatusCode::TOO_MANY_REQUESTS {
            false
        } else if let Some(value) = response.headers().get("Retry-After") {
            let str = value.to_str().unwrap();
            let retry_after = str.parse::<f64>().unwrap();

            time::sleep(Duration::from_secs_f64(retry_after)).await;

            true
        } else {
            false
        }
    }
}

impl Default for Ratelimit {
    fn default() -> Self {
        Self {
            limit: i64::MAX,
            remaining: i64::MAX,
            reset: None,
            reset_after: None
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RatelimitRequest {
    method: String,
    headers: HashMap<String, String>,
    body: String,
}

impl RatelimitRequest {
    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}
