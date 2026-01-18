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

use std::sync::{Arc, atomic::Ordering};

use hartex_discord_grpc::manager::{
    IdentifyRequest, ReadyResponse, ShardAssignment, WhoamiRequest, WhoamiResponse,
    WorkerSessionStartLimit, manager_server::Manager,
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, async_trait};
use tracing::instrument;
use twilight_model::{gateway::connection_info::BotConnectionInfo, user::CurrentUser};

use crate::state::{ManagerServerState, Worker};

pub struct ManagerServerImpl {
    state: Arc<Mutex<ManagerServerState>>,
}

impl ManagerServerImpl {
    pub fn new(info: BotConnectionInfo, current_user: CurrentUser) -> Self {
        Self {
            state: Arc::new(Mutex::new(ManagerServerState::new(info, current_user))),
        }
    }
}

#[async_trait]
impl Manager for ManagerServerImpl {
    #[instrument(skip_all)]
    async fn identify(
        &self,
        request: Request<IdentifyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let identify = request.into_inner();

        let mut locked = self.state.lock().await;
        let worker_id = locked.next_worker_id.fetch_add(1, Ordering::SeqCst);

        let for_this_shard: Vec<_> = locked
            .all_shards
            .difference(&locked.assigned_shards)
            .take(identify.capacity as usize)
            .copied()
            .collect();

        locked.assigned_shards.extend(for_this_shard.clone());
        tracing::info!(
            "worker ID: {worker_id}, initial assignment: {:?}",
            &for_this_shard
        );

        let initial_assignments: Vec<_> = for_this_shard
            .iter()
            .map(|shard_id| ShardAssignment {
                shard_id: *shard_id,
                shard_count: u32::try_from(locked.all_shards.len()).unwrap(),
                resume_data: None,
            })
            .collect();

        let worker = Worker::new(
            worker_id,
            identify.capacity,
            Some(initial_assignments.clone()),
        );
        locked.workers.insert(worker_id, worker);

        Ok(Response::new(ReadyResponse {
            worker_id,
            initial_assignments,
            session_start_limit: Some(WorkerSessionStartLimit {
                max_concurrency: u32::from(locked.session_start_limit.max_concurrency),
                remaining: locked.session_start_limit.remaining,
                reset_after: locked.session_start_limit.reset_after,
                total: locked.session_start_limit.total,
            }),
        }))
    }

    async fn whoami(&self, _: Request<WhoamiRequest>) -> Result<Response<WhoamiResponse>, Status> {
        let locked = self.state.lock().await;
        Ok(Response::new(WhoamiResponse {
            username: locked.current_user.name.clone(),
            discriminator: locked.current_user.discriminator().to_string(),
            user_id: locked.current_user.id.to_string(),
        }))
    }
}
