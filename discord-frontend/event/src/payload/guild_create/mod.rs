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

use std::collections::HashMap;
use std::env as stdenv;

use base::discord::model::gateway::payload::incoming::GuildCreate;
use base::error::{Error, ErrorKind, Result};
use hyper::{Body, Method, Request};
use base::discord::http::routing::Route;
use cache_discord::DiscordCache;
use loadbal::Request as LoadbalRequest;
use manidb::whitelist::GetGuildWhitelistStatus;
use rest::request::RatelimitRequest;

pub async fn handle_guild_create(payload: Box<GuildCreate>, loadbal_port: u64) -> Result<()> {
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
                kind: ErrorKind::EnvVarError { src }
            });
        }

        let mut token = result.unwrap();
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }
        let _ = async move {
            log::error!("guild `{}` (id {}) is not whitelisted. attempting to leave the guild", &payload.name, payload.id);

            let mut headers = HashMap::new();
            headers.insert(String::from("Authorization"), token.clone());

            let rl_request = RatelimitRequest {
                body: String::new(),
                method: Method::POST.to_string(),
                headers,
            };
            let serde_result = serde_json::to_string(&rl_request);
            if let Err(src) = serde_result {
                return Err(Error {
                    kind: ErrorKind::JsonError { src }
                });
            }
            let body = serde_result.unwrap();

            let loadbal_request = LoadbalRequest {
                target_server_type: String::from("rest"),
                method: Method::POST.to_string(),
                route: Route::LeaveGuild { guild_id: payload.id.get() }.to_string(),
                headers: HashMap::new(),
                body,
            };
            let serde_result = serde_json::to_string(&loadbal_request);
            if let Err(src) = serde_result {
                return Err(Error {
                    kind: ErrorKind::JsonError { src }
                });
            }
            let actual_json = serde_result.unwrap();

            let result = Request::builder()
                .method(Method::POST)
                .uri(format!("http://127.0.0.1:{loadbal_port}/request"))
                .body(Body::from(actual_json));
            if let Err(src) = result {
                return Err(Error {
                    kind: ErrorKind::HttpError { src }
                });
            }

            Ok(())
        };

        todo!()
    }

    Ok(())
}
