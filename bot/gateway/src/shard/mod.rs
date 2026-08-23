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
    pin::Pin,
    task::{Context, Poll},
};

use futures::StreamExt;
pub use handle::ShardHandle;
use twilight_gateway::Shard;

use crate::error::GatewayResult;

mod handle;

pub struct ShardFuture {
    shard: Shard,
}

impl ShardFuture {
    pub fn new(shard: Shard) -> Self {
        Self { shard }
    }
}

impl Future for ShardFuture {
    type Output = GatewayResult<()>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
