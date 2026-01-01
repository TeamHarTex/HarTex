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
    sync::{Arc, atomic::Ordering},
};

use hartex_discord_grpc::manager::{
    IdentifyRequest, ReadyResponse, ShardAssignment, manager_server::Manager,
};
use tonic::{Request, Response, Status, async_trait};
use twilight_model::gateway::connection_info::BotConnectionInfo;

use crate::state::{ManagerServerState, Worker};

pub struct ManagerServerImpl {
    state: Arc<ManagerServerState>,
}

impl ManagerServerImpl {
    pub fn new(info: BotConnectionInfo) -> Self {
        Self {
            state: Arc::new(ManagerServerState::new(info)),
        }
    }

    pub fn assigned_shards(&self) -> HashSet<u32> {
        self.state
            .workers
            .iter()
            .fold(HashSet::new(), |set, worker| {
                set.union(
                    &worker
                        .shard_assignments
                        .iter()
                        .map(|assignment| assignment.shard_id)
                        .collect(),
                )
                .copied()
                .collect()
            })
    }
}

#[async_trait]
impl Manager for ManagerServerImpl {
    async fn identify(
        &self,
        request: Request<IdentifyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let identify = request.into_inner();
        let next = self.state.next_worker_id.lock().await;

        let worker_id = next.load(Ordering::SeqCst);
        next.store(worker_id + 1, Ordering::SeqCst);

        let worker = Worker::new(worker_id, identify.capacity);
        self.state.workers.insert(worker_id, worker);

        let already_assigned = self.assigned_shards();
        let initial_assigned = self
            .state
            .all_shards
            .difference(&already_assigned)
            .take(identify.capacity as usize);

        Ok(Response::new(ReadyResponse {
            worker_id,
            initial_assignments: initial_assigned
                .map(|shard_id| ShardAssignment {
                    shard_id: *shard_id,
                    shard_count: self.state.all_shards.len() as u32,
                    resume_data: None,
                })
                .collect(),
        }))
    }
}
