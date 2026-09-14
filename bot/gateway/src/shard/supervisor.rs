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

use tokio::task::JoinSet;
use twilight_gateway::{Command, Config, Shard};
use twilight_model::gateway::ShardId;

use crate::{
    error::GatewayResult,
    shard::{ShardFuture, ShardHandle, ShardTermination, termination::TerminationReason},
};

pub struct ShardSupervisor {
    id: ShardId,
    config: Config,
    handle: Option<ShardHandle>,
}

impl ShardSupervisor {
    pub fn new(id: ShardId, config: Config) -> Self {
        Self {
            id,
            config,
            handle: None,
        }
    }

    pub fn handle_termination(
        &mut self,
        termination: ShardTermination,
        _: &mut JoinSet<ShardTermination>,
    ) {
        match termination.reason {
            TerminationReason::Disconnected => {
                tracing::info!("shard {} disconnected", self.id.number());
            }
            TerminationReason::Error(error) => {
                tracing::error!(
                    "shard {} encountered an error: {}",
                    self.id.number(),
                    error
                );
            }
            TerminationReason::Reconnect => {
                tracing::info!("shard {} instructed to reconnect", self.id.number());
            }
            TerminationReason::Resume => {
                tracing::info!("shard {} instructed to resume", self.id.number());
            }
            TerminationReason::Shutdown => {
                tracing::info!("shard {} instructed to shut down", self.id.number());
            }
        }
    }

    pub fn send(&self, command: impl Command) -> GatewayResult<()> {
        if let Some(ref handle) = self.handle {
            handle.send(command)?;
        }

        Ok(())
    }

    pub fn spawn(&mut self, tasks: &mut JoinSet<ShardTermination>) {
        let shard = Shard::with_config(self.id, self.config.clone());
        self.handle = Some(ShardHandle::new(shard.sender()));

        tasks.spawn(ShardFuture::new(shard));
    }
}
