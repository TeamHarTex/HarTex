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
use hartex_model::env::StartupEnv;

/// # Asynchronous Function `pre_startup`
///
/// Returns the cluster, client and event stream as constructed with the environments.
///
/// ## Parameters
/// - `environment`, type `Environment`: the environment to construct the return values
pub async fn pre_startup(environment: StartupEnv) -> (Cluster, Client, Events, InMemoryCache) {
    let shard_scheme = ShardScheme::Auto;
    let intents = Intents::all();

    if environment.application_id.is_none() {
        tracing::warn!("`APPLICATION_ID` is not specified, exiting");

        process::exit(-1);
    }

    if environment.token.is_none() {
        tracing::warn!("`BOT_TOKEN` is not specified, exiting");

        process::exit(-1);
    }

    tracing::trace!("building http client");

    let http = Client::builder()
        .application_id(ApplicationId::from(
            environment.application_id.unwrap().parse::<u64>().unwrap()
        ))
        .token(environment.token.clone().unwrap())
        .build();

    tracing::trace!("building bot cluster");
    tracing::trace!("registering gateway intents [all]");

    let result = Cluster::builder(environment.token.unwrap(), intents)
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
