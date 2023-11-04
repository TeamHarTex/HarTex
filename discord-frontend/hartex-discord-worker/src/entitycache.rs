/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

#![allow(unused_variables)]

use hartex_discord_core::discord::model::gateway::event::DispatchEvent;
use hartex_discord_core::discord::model::gateway::event::GatewayEvent;
use hartex_discord_entitycache_cacheupdaters::CacheUpdater;
use hartex_log::log;
use miette::IntoDiagnostic;

/// Update entity cache.
#[allow(clippy::unused_async)]
pub async fn update(event: GatewayEvent) -> miette::Result<()> {
    let GatewayEvent::Dispatch(_, dispatch) = event else {
        return Ok(());
    };

    match dispatch {
        DispatchEvent::GuildCreate(guild_create) => {
            log::trace!("updating cache using GUILD_CREATE event");
            guild_create.update().await.into_diagnostic()?;
        }
        DispatchEvent::MemberChunk(member_chunk) => {
            log::trace!("updating cache using GUILD_MEMBER_CHUNK event");
            member_chunk.update().await.into_diagnostic()?;
        }
        _ => (),
    }

    Ok(())
}
