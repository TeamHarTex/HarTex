/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
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

//! # The `pre_startup` Module
//!
//! This module provides pre-startup procedures for the bot.

use std::process;

use hartex_base::{
    discord::{
        cache_inmemory::{
            CloneableInMemoryCache,
            InMemoryCache,
            ResourceType
        },
        gateway::{
            cluster::{
                Events,
                ShardScheme
            },
            CloneableCluster,
            Cluster,
            EventTypeFlags,
            Intents
        },
        http::{
            Client,
            CloneableClient
        },
        model::id::ApplicationId
    },
    logging::tracing
};
use hartex_env::StartupEnv;

/// # Asynchronous Function `pre_startup`
///
/// Returns the cluster, client and event stream as constructed with the environments.
///
/// ## Parameters
/// - `environment`, type `Environment`: the environment to construct the return values
#[allow(clippy::missing_panics_doc)]
pub async fn pre_startup(
    environment: StartupEnv
) -> (
    CloneableCluster,
    CloneableClient,
    Events,
    CloneableInMemoryCache
) {
    let shard_scheme = ShardScheme::Auto;
    let intents = Intents::all();

    if environment.application_aid.is_none() {
        tracing::warn!("`APPLICATION_ID` is not specified, exiting");

        process::exit(-1);
    }

    if environment.bot_token.is_none() {
        tracing::warn!("`BOT_TOKEN` is not specified, exiting");

        process::exit(-1);
    }

    tracing::trace!("building http client");

    let http = Client::builder()
        .application_id(ApplicationId(environment.application_aid.unwrap()))
        .token(environment.bot_token.clone().unwrap())
        .build();
    let client = CloneableClient::new(http);

    tracing::trace!("building bot cluster");
    tracing::trace!("registering gateway intents [all]");

    let result = Cluster::builder(environment.bot_token.unwrap(), intents)
        .event_types(EventTypeFlags::all())
        .http_client(client.clone().0)
        .shard_scheme(shard_scheme)
        .build()
        .await;

    if let Err(ref error) = result {
        tracing::error!("failed to build bot cluster: {error}");

        process::exit(-1);
    }

    let result = result.unwrap();

    tracing::trace!("building in-memory cache");

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::all())
        .build();

    (
        CloneableCluster::new(result.0),
        client,
        result.1,
        CloneableInMemoryCache::new(cache)
    )
}
