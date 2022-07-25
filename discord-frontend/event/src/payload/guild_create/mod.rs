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

use hyper::Client;
use std::env as stdenv;

use base::discord::model::gateway::payload::incoming::GuildCreate;
use base::error::{Error, ErrorKind, Result};
use cache_discord::DiscordCache;
use manidb::whitelist::GetGuildWhitelistStatus;

mod extras;

pub async fn handle_guild_create(payload: Box<GuildCreate>, loadbal_port: u16) -> Result<()> {
    log::info!(
        "joined a new guild `{}` (id {}); checking its whitelist status",
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

        log::trace!("caching guild");
        if let Err(error) = DiscordCache.update(&*payload).await {
            log::trace!("failed to cache guild {error:?}")
        }
    } else {
        let result = stdenv::var("BOT_TOKEN");
        if let Err(src) = result {
            return Err(Error {
                kind: ErrorKind::EnvVarError { src },
            });
        }

        let mut token = result.unwrap();
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        tokio::spawn(Client::new().request(extras::leave_guild(payload, token, loadbal_port)?));
    }

    Ok(())
}
