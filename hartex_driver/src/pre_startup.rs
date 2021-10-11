//! # The `pre_startup` Module
//!
//! This module provides pre-startup procedures for the bot.

use std::process;

use hartex_core::{
    discord::{
        cache_inmemory::{
            InMemoryCache,
            ResourceType
        },
        gateway::{
            cluster::{
                Events,
                ShardScheme
            },
            Cluster,
            EventTypeFlags,
            Intents
        },
        http::Client,
        model::id::ApplicationId
    },
    logging::tracing
};

use crate::env_setup;

/// # Asynchronous Function `pre_startup`
///
/// Returns the cluster, client and event stream as constructed with the environments.
///
/// ## Parameters
/// - `environment`, type `Environment`: the environment to construct the return values
pub async fn pre_startup(
    environment: env_setup::Environment
) -> (Cluster, Client, Events, InMemoryCache) {
    let shard_scheme = ShardScheme::Auto;
    let intents = Intents::all();

    tracing::trace!("building http client");

    let http = Client::builder()
        .application_id(ApplicationId::from(
            environment.application_id.parse::<u64>().unwrap()
        ))
        .token(environment.token.clone())
        .build();

    tracing::trace!("building bot cluster");
    tracing::trace!("registering gateway intents [all]");

    let result = Cluster::builder(environment.token, intents)
        .event_types(EventTypeFlags::all())
        .http_client(http.clone())
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

    (result.0, http, result.1, cache)
}
