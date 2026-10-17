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

use std::str::FromStr;

use protocol::buffers::gateway::RequestGuildMembers;
use shared_types::id::{GuildId, UserId};
use twilight_model::gateway::{
    OpCode,
    payload::outgoing::{
        RequestGuildMembers as TwilightRequestGuildMembers,
        request_guild_members::{RequestGuildMemberId, RequestGuildMembersInfo},
    },
};

use crate::error::{GatewayError, GatewayResult};

pub fn request_guild_members(
    request: RequestGuildMembers,
) -> GatewayResult<TwilightRequestGuildMembers> {
    let length = request.user_ids.len();
    let user_ids = match length {
        0 => None,
        1 => Some(RequestGuildMemberId::One(UserId::from_str(
            &request.user_ids[0],
        )?)),
        2.. => Some(RequestGuildMemberId::Multiple(
            request
                .user_ids
                .into_iter()
                .map(|id| UserId::from_str(&id).map_err(GatewayError::from))
                .collect::<GatewayResult<Vec<UserId>>>()?,
        )),
    };

    Ok(TwilightRequestGuildMembers {
        op: OpCode::RequestGuildMembers,
        d: RequestGuildMembersInfo {
            guild_id: GuildId::from_str(&request.guild_id)?,
            limit: request.limit,
            nonce: request.nonce,
            presences: request.presences,
            query: request.query,
            user_ids,
        },
    })
}
