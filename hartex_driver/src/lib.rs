//! # `hartex_driver` - The "Main Function" of HarTex Discord bot
//!
//! This `hartex_driver` crate contains effectively the "main function" of the bot as well as some
//! "moving pieces" that are required for the bot to work.

#![feature(format_args_capture)]

use futures_util::future::Either;

use tokio_stream::StreamExt;

use hartex_cmdsys::framework::CommandFramework;

use hartex_core::{
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
pub mod ctrlc;
pub mod env_setup;
pub mod events;
pub mod handler;
pub mod interactions;
pub mod pre_startup;

/// # Asynchronous Function `hartex_main`
///
/// This is the main entry point of HarTex Discord Bot.
pub async fn hartex_main() -> HarTexResult<()> {
    let span = tracing::trace_span!("environment setup");
    let environment = span.in_scope(env_setup::environment_setup);

    let span = tracing::trace_span!("pre-startup phase");
    let (cluster, http, events, cache) = pre_startup::pre_startup(environment)
        .instrument(span)
        .await;

    let span = tracing::trace_span!("ctrlc handler");
    span.in_scope(ctrlc::ctrlc_handler);

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