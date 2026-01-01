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
use std::{
    collections::HashSet,
    sync::{Arc, atomic::AtomicU32},
};

use dashmap::DashMap;
use hartex_discord_grpc::manager::ShardAssignment;
use tokio::sync::Mutex;
use twilight_model::gateway::connection_info::BotConnectionInfo;

pub struct ManagerServerState {
    pub next_worker_id: Arc<Mutex<AtomicU32>>,
    pub all_shards: HashSet<u32>,
    pub workers: DashMap<u32, Worker>,
}

impl ManagerServerState {
    pub fn new(info: BotConnectionInfo) -> Self {
        Self {
            next_worker_id: Arc::new(Mutex::new(AtomicU32::new(0))),
            all_shards: (0..info.shards).collect(),
            workers: DashMap::new(),
        }
    }
}

pub struct Worker {
    #[expect(dead_code)]
    pub capacity: u32,
    #[expect(dead_code)]
    pub id: u32,
    pub shard_assignments: HashSet<ShardAssignment>,
}

impl Worker {
    pub fn new(id: u32, capacity: u32) -> Self {
        Self {
            capacity,
            id,
            shard_assignments: HashSet::new(),
        }
    }
}
