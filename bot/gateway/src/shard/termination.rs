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

use twilight_model::gateway::ShardId;

use crate::error::GatewayError;

pub enum TerminationReason {
    Disconnected,
    Error(GatewayError),
    Reconnect,
    SessionInvalidated,
    Shutdown,
}

pub struct ShardTermination {
    pub(crate) id: ShardId,
    pub(crate) reason: TerminationReason,
}

impl ShardTermination {
    fn new(id: ShardId, reason: TerminationReason) -> Self {
        Self { id, reason }
    }

    pub fn disconnected(id: ShardId) -> Self {
        Self::new(id, TerminationReason::Disconnected)
    }

    pub fn error(id: ShardId, error: GatewayError) -> Self {
        Self::new(id, TerminationReason::Error(error))
    }

    pub fn reconnect(id: ShardId) -> Self {
        Self::new(id, TerminationReason::Reconnect)
    }

    pub fn session_invalidated(id: ShardId) -> Self {
        Self::new(id, TerminationReason::SessionInvalidated)
    }

    pub fn shutdown(id: ShardId) -> Self {
        Self::new(id, TerminationReason::Shutdown)
    }
}
