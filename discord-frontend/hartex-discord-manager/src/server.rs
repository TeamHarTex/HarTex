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

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use hartex_discord_grpc::manager::{IdentifyRequest, ReadyResponse, manager_server::Manager};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, async_trait};
use twilight_model::gateway::connection_info::BotConnectionInfo;

pub struct ManagerServerImpl {
    state: Arc<ManagerServerState>,
}

impl ManagerServerImpl {
    pub fn new(info: BotConnectionInfo) -> Self {
        Self {
            state: Arc::new(ManagerServerState::new(info)),
        }
    }
}

#[async_trait]
impl Manager for ManagerServerImpl {
    async fn identify(
        &self,
        request: Request<IdentifyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let _ = request.into_inner();
        let next = self.state.next_worker_id.lock().await;

        let worker_id = next.load(Ordering::SeqCst);
        next.store(worker_id + 1, Ordering::SeqCst);

        Ok(Response::new(ReadyResponse {
            worker_id,
            initial_assignments: vec![],
        }))
    }
}

struct ManagerServerState {
    next_worker_id: Arc<Mutex<AtomicU32>>,
    #[expect(dead_code)]
    total_shards: u32,
}

impl ManagerServerState {
    fn new(info: BotConnectionInfo) -> Self {
        Self {
            next_worker_id: Arc::new(Mutex::new(AtomicU32::new(0))),
            total_shards: info.shards,
        }
    }
}
