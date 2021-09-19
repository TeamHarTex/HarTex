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
                ShardScheme
            },
            EventTypeFlags,
            Intents
        },
        http::Client,
        model::id::ApplicationId
    },
    error::HarTexResult,
    events::EventType
};

use hartex_eventsys::emitter::EventEmitter;

use hartex_logging::Logger;

pub mod commands;
pub mod events;
pub mod handler;
pub mod interactions;

/// # Asynchronous Function `hartex_main`
///
/// This is the main entry point of HarTex Discord Bot.
pub async fn hartex_main() -> HarTexResult<()> {
    // loads the .env file to obtain environment variables
    dotenv::dotenv().ok();

    Logger::verbose(
        "loaded environment variables",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

    // obtains the token from the environment variables
    let token = match env::var("HARTEX_TOKEN") {
        Ok(token) => token,
        Err(var_error) => {
            Logger::error(
                format!("could not retrieve the bot token from the environment due to an error: {var_error}"),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            process::exit(-1)
        }
    };

    let application_id = match env::var("APPLICATION_ID") {
        Ok(id) => id,
        Err(var_error) => {
            Logger::error(
                format!("could not retrieve the application id from the environment due to an error: {var_error}"),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            process::exit(-1)
        }
    };

    Logger::verbose(
        "successfully retrieved bot token",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

    let shard_scheme = ShardScheme::Auto;

    Logger::verbose(
        "building bot cluster",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );
    Logger::verbose(
        "registering gateway intents [all]",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

    let intents = Intents::all();

    let http = Client::builder()
        .application_id(ApplicationId::from(application_id.parse::<u64>().unwrap()))
        .token(token.clone())
        .build();

    let (cluster, events) = Cluster::builder(token, intents)
        .event_types(EventTypeFlags::all())
        .http_client(http.clone())
        .shard_scheme(shard_scheme)
        .build()
        .await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    Logger::verbose(
        "building http client",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

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

    Logger::verbose(
        "building in-memory cache",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::all())
        .build();

    Logger::verbose(
        "registering ctrl-c handler",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

    ctrlc::set_handler(|| {
        Logger::warn(
            "ctrl-c signal received; terminating process",
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        process::exit(0)
    })?;

    let mut events = events.map(Either::Left).merge(framework_events.map(Either::Right));

    while let Some(event) = events.next().await {
        match event {
            Either::Left((_, twilight)) => {
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
