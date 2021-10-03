//! # `hartex_driver` - The "Main Function" of HarTex Discord bot
//!
//! This `hartex_driver` crate contains effectively the "main function" of the bot as well as some
//! "moving pieces" that are required for the bot to work.

#![feature(format_args_capture)]

use std::{
    env,
    process
};

use futures_util::future::Either;

use tokio_stream::StreamExt;

use hartex_cmdsys::framework::CommandFramework;

use hartex_core::{
    ctrlc,
    discord::{
        cache_inmemory::{
            InMemoryCache,
            ResourceType
        },
        gateway::{
            cluster::{
                Cluster,
                Events,
                ShardScheme
            },
            EventTypeFlags,
            Intents
        },
        http::Client,
        model::id::ApplicationId
    },
    error::HarTexResult,
    events::EventType,
    logging::tracing::{
        self,
        Instrument
    }
};

use hartex_eventsys::emitter::EventEmitter;

use hartex_logging::Logger;

pub mod commands;
pub mod env_setup;
pub mod events;
pub mod handler;
pub mod interactions;

/// # Asynchronous Function `hartex_main`
///
/// This is the main entry point of HarTex Discord Bot.
pub async fn hartex_main() -> HarTexResult<()> {
    let span = tracing::trace_span!("environment setup");
    let environment = span.in_scope(env_setup::environment_setup);

    let span = tracing::trace_span!("pre-startup phase");
    let (cluster, http, events, cache) = pre_startup(environment)
        .instrument(span)
        .await;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    Logger::verbose(
        "initializing command framework",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );
    let framework = CommandFramework::default();

    let listeners = framework.clone().listeners();
    let emitter = EventEmitter::new(listeners);

    let framework_events = framework.events();

    let mut events = events.map(Either::Left).merge(framework_events.map(Either::Right));

    while let Some(event) = events.next().await {
        match event {
            Either::Left((_, twilight)) => {
                cache.update(&twilight);

                tokio::spawn(events::handle_event(
                    (EventType::Twilight, Some(twilight), None),
                    http.clone(),
                    emitter.clone(),
                    cache.clone(),
                    cluster.clone()
                ));
            }
            Either::Right(custom) => {
                tokio::spawn(events::handle_event(
                    (EventType::Custom, None, Some(custom)),
                    http.clone(),
                    emitter.clone(),
                    cache.clone(),
                    cluster.clone()
                ));
            }
        }
    }

    Ok(())
}

/// # Asynchronous Function `pre_startup`
///
/// Returns the cluster, client and event stream as constructed with the environments.
///
/// ## Parameters
/// - `environment`, type `Environment`: the environment to construct the return values
async fn pre_startup(environment: env_setup::Environment) -> (Cluster, Client, Events, InMemoryCache) {
    let shard_scheme = ShardScheme::Auto;
    let intents = Intents::all();

    tracing::trace!("building http client");

    let http = Client::builder()
        .application_id(ApplicationId::from(environment.application_id.parse::<u64>().unwrap()))
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

    tracing::trace!("registering ctrl-c handler");

    if let Err(error) = ctrlc::set_handler(|| {
        let span = tracing::warn_span!("ctrl-c handler");
        span.in_scope(|| {
            tracing::warn!("ctrl-c signal received; terminating process");

            process::exit(0);
        });
    }) {
        tracing::error!("failed to set ctrl-c handler: {error}");

        process::exit(-1);
    }

    (result.0, http, result.1, cache)
}