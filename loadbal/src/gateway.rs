/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! WebSocket Gateway for servers to load balance to connect to.

use futures_util::StreamExt;
use tokio::net::TcpStream;

pub async fn handle_connection(stream: TcpStream) {
    log::trace!("opening websocket gateway for {:?}", stream.local_addr().ok());
    let result = tokio_tungstenite::accept_async(stream).await;
    if let Err(error) = result {
        log::error!("failed to open websocket for {:?}: {error}", stream.local_addr().ok());
        return;
    }

    let websocket = result.unwrap();
    let (outgoing, mut incoming) = websocket.split();

    let expected_identify = incoming.next().await;
}
