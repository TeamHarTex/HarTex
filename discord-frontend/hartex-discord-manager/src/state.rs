/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use std::sync::{Arc, atomic::AtomicU32};

use dashmap::DashMap;
use tokio::sync::Mutex;
use twilight_model::gateway::connection_info::BotConnectionInfo;

pub struct ManagerServerState {
    pub next_worker_id: Arc<Mutex<AtomicU32>>,
    #[expect(dead_code)]
    pub total_shards: u32,
    #[expect(dead_code)]
    pub workers: DashMap<u32, Worker>,
}

impl ManagerServerState {
    pub fn new(info: BotConnectionInfo) -> Self {
        Self {
            next_worker_id: Arc::new(Mutex::new(AtomicU32::new(0))),
            total_shards: info.shards,
            workers: DashMap::new(),
        }
    }
}

pub struct Worker;
