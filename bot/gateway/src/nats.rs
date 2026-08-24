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

use async_nats::{Client, Subscriber};
use futures_util::Stream;

use crate::error::GatewayResult;

const GATEWAY_COMMANDS: &'static str = "gateway.commands";

pub struct GatewayCommandStream {
    subscriber: Subscriber,
}

impl GatewayCommandStream {
    pub async fn new(client: Client) -> GatewayResult<Self> {
        Ok(Self {
            subscriber: client.subscribe(GATEWAY_COMMANDS).await?,
        })
    }
}

impl Stream for GatewayCommandStream {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
