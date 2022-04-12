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

use futures_util::{SinkExt, StreamExt};
use loadbal::payload::{ErrorType, InvalidSession, Payload, PayloadInner, Welcome};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_connection(stream: TcpStream) {
    log::trace!(
        "opening websocket gateway for {:?}",
        stream.local_addr().ok()
    );
    let result = tokio_tungstenite::accept_async(stream).await;
    if let Err(error) = result {
        log::error!("failed to open websocket: {error}");
        return;
    }

    let websocket = result.unwrap();
    let (mut outgoing, mut incoming) = websocket.split();

    let expected_identify = incoming.next().await;
    if expected_identify.is_none() || expected_identify.is_some_and(|result| result.is_err()) {
        let invalid_session = Payload {
            opcode: 1,
            payload: PayloadInner::PayloadInvalidSession(InvalidSession {
                error_type: ErrorType::IdentifyNotReceived,
            }),
        };
        outgoing
            .send(Message::Text(
                serde_json::to_string(&invalid_session).unwrap(),
            ))
            .await;

        return;
    }

    // todo: register server to load balance

    let welcome = Payload {
        opcode: 2,
        payload: PayloadInner::PayloadWelcome(Welcome {
            heartbeat_interval_ms: 30500,
        }),
    };
    outgoing.send(Message::Text(serde_json::to_string(&welcome).unwrap()));
}
