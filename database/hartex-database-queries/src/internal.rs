/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::pin::Pin;

use tokio::net::TcpStream;
use wtx::database::client::postgres::Config;
use wtx::database::client::postgres::Executor;
use wtx::database::client::postgres::ExecutorBuffer;
use wtx::misc::Uri;
use wtx::misc::Xorshift64;
use wtx::misc::simple_seed;

pub(crate) type Ret<'a> = Pin<
    Box<dyn Future<Output = wtx::Result<Executor<wtx::Error, ExecutorBuffer, TcpStream>>> + Send + 'a>,
>;

pub(crate) fn __internal_executor_constructor(uri: Uri<&str>) -> Ret {
    let mut rng = Xorshift64::from(simple_seed());

    Box::pin(async move {
        Executor::connect(
            &Config::from_uri(&uri)?,
            ExecutorBuffer::new(usize::MAX, &mut rng),
            &mut rng,
            TcpStream::connect(uri.hostname_with_implied_port()).await?,
        )
        .await
    })
}
