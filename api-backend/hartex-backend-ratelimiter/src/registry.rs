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

use std::collections::HashMap;
use std::intrinsics::type_name;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::RwLock;

use governor::Quota;
use governor::clock::DefaultClock;
use governor::middleware::StateInformationMiddleware;
use governor::state::keyed::DefaultKeyedStateStore;
use governor::state::RateLimiter;
use lazy_static::lazy_static;
use rocket::http::Method;

pub(crate) struct Registry {
    limiter_map: RwLock<HashMap<Method, HashMap<String, RegisteredRateLimiter>>>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            limiter_map: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn get_or_insert<T>(method: Method, route: &str, quota: Quota) -> RegisteredRateLimiter {
        let route_name = type_name::<T>().to_string() + "::" + route;

        let option_limiter = if let Ok(readlock) = REGISTRY.limiter_map.read() {
            if let Some(found_method) = readlock.get(&method) {
                found_method.get(&route_name).map(|limiter| {
                    Arc::clone(limiter)
                })
            } else {
                None
            }
        } else {
            None
        };

        let limiter = if let Some(limiter) = option_limiter {
            limiter
        } else {
            let mut writelock = REGISTRY.limiter_map.write().unwrap();

            if let Some(found_method) = writelock.get_mut(&method) {
                if let Some(limiter) = found_method.get(&route_name) {
                    Arc::clone(limiter)
                } else {
                    let limiter = Arc::new(RateLimiter::keyed(quota).with_middleware::<StateInformationMiddleware>());
                    found_method.insert(route_name, Arc::clone(&limiter));

                    limiter
                }
            } else {
                let mut limiter_map = HashMap::new();
                let limiter = Arc::new(RateLimiter::keyed(quota).with_middleware::<StateInformationMiddleware>());
                limiter_map.insert(route_name, Arc::clone(&limiter));
                writelock.insert(method, limiter_map);

                limiter
            }
        };

        limiter.retain_recent();
        limiter.shrink_to_fit();

        limiter
    }
}

pub(crate) type RegisteredRateLimiter = Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock, StateInformationMiddleware>>;

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
}
