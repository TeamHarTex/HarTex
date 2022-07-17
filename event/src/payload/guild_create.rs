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

use base::discord::model::gateway::payload::incoming::GuildCreate;
use base::error::Result;
use manidb::whitelist::GetGuildWhitelistStatus;

pub async fn handle_guild_create(payload: Box<GuildCreate>) -> Result<()> {
    log::info!(
        "joined a new guild `{}` (id {}); checking its whitelist status...",
        &payload.name,
        payload.id
    );

    let result = GetGuildWhitelistStatus::new(payload.id).await;
    if let Err(error) = result {
        log::error!("failed to check whitelist status: {error}");
        return Err(error);
    }

    let option = result.unwrap();
    if let Some(whitelist) = option {
        log::info!(
            "guild `{}` (id {}) is whitelisted and is so since {}",
            &whitelist.name,
            whitelist.id,
            whitelist.whitelisted_since
        );
    } else {
        todo!()
    }

    Ok(())
}
