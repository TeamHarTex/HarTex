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

//! Websocket payloads for each connection.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub opcode: u8,
    pub payload: PayloadInner,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum PayloadInner {
    PayloadIdentify(Identify),
    PayloadInvalidSession(InvalidSession),
    PayloadWelcome(Welcome),
}

#[derive(Deserialize, Serialize)]
pub struct Identify {
    pub server_type: ServerType,
}

#[derive(Deserialize, Serialize)]
pub struct InvalidSession {
    pub error_type: ErrorType,
}

#[derive(Deserialize, Serialize)]
pub struct Welcome {
    pub heartbeat_interval_ms: u16,
}

#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum ErrorType {
    IdentifyNotReceived,
}

#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum ServerType {
    Event,
}
