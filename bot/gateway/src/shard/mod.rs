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

use futures_util::future::BoxFuture;
pub use handle::ShardHandle;
use twilight_gateway::{EventTypeFlags, Shard, StreamExt};
use twilight_model::gateway::CloseFrame;

use crate::error::GatewayResult;

mod handle;

pub struct ShardFuture {
    fut: BoxFuture<'static, GatewayResult<()>>,
}

impl ShardFuture {
    pub fn new(mut shard: Shard) -> Self {
        let fut = Box::pin(async move {
            while let Some(result) = shard.next_event(EventTypeFlags::all()).await {
                match result {
                    Ok(event) => {},
                    Err(err) => {},
                }
            }

            shard.close(CloseFrame::RESUME);

            Ok(())
        });

        Self { fut }
    }
}

impl Future for ShardFuture {
    type Output = GatewayResult<()>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().fut.as_mut().poll(ctx)
    }
}
