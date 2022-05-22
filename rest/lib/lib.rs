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

#![deny(warnings)]
#![feature(let_else)]

use std::sync::Arc;

use dashmap::DashMap;
use hyper::Client;
use hyper_rustls::HttpsConnector;
use hyper_trust_dns::TrustDnsHttpConnector;
use tokio::sync::{Mutex, RwLock};

use crate::request::Ratelimit;

pub mod request;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct RestState {
    pub client: Client<HttpsConnector<TrustDnsHttpConnector>>,
    pub ratelimit: RatelimitManager,
}

impl RestState {
    pub fn new(client: Client<HttpsConnector<TrustDnsHttpConnector>>) -> Self {
        Self {
            client,
            ratelimit: RatelimitManager::new(),
        }
    }
}

#[derive(Clone)]
pub struct RatelimitManager {
    pub buckets: Arc<RwLock<DashMap<String, Arc<Mutex<Ratelimit>>>>>,
    pub global: Arc<Mutex<()>>,
}

impl RatelimitManager {
    pub fn new() -> Self {
        Self::__internal_new()
    }

    pub(in crate) fn __internal_new() -> Self {
        Self {
            buckets: Arc::default(),
            global: Arc::default(),
        }
    }
}

impl Default for RatelimitManager {
    fn default() -> Self {
        Self::new()
    }
}
