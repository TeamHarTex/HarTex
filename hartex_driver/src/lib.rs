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
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex; if not, see <https://www.gnu.org/licenses/>.
 */

//! # `hartex_driver` - The "Main Function" of `HarTex` Discord bot
//!
//! This `hartex_driver` crate contains effectively the "main function" of the bot as well as some
//! "moving pieces" that are required for the bot to work.

#![deny(clippy::pedantic, warnings, unsafe_code)]
#![feature(once_cell)]

use std::{
    lazy::SyncLazy,
    panic::{
        self,
        PanicInfo
    }
};

use futures_util::future::Either;
use hartex_core::{
    error::HarTexResult,
    events::EventType,
    logging::tracing::{
        self,
        Instrument
    },
    HARTEX_BUILD
};
use hartex_env::StartupEnv;
use tokio_stream::StreamExt;

pub mod commands;
pub mod ctrlc;
pub mod events;
pub mod fw_setup;
pub mod handler;
pub mod interactions;
pub mod pre_startup;

/// # Static `BUG_REPORT_URL`
///
/// The bug report url for the bot when an Internal Bot Error occurs.
pub static BUG_REPORT_URL: &str = "https://github.com/HarTexTeam/HarTex-rust-discord-bot/issues/new?assignees=&labels=Bot%3A+Bug%2CBot%3A+IBE&template=internal-bot-error.yml";

/// # Static `RUST_DEFAULT_PANIC_HOOK`
///
/// The default panic hook for Rust.
pub static RUST_DEFAULT_PANIC_HOOK: SyncLazy<Box<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static>> =
    SyncLazy::new(|| {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(|info| {
            let span = tracing::error_span!(parent: None, "panic handler");
            span.in_scope(|| {
                tracing::error!("unexpected panic occurred, invoking panic handler...");
                report_ibe(BUG_REPORT_URL);
            });

            (*RUST_DEFAULT_PANIC_HOOK)(info);
        }));

        hook
    });

/// # Asynchronous Function `hartex_main`
///
/// This is the main entry point of `HarTex` Discord Bot.
///
/// ## Errors
///
/// Returns bot-related errors.
pub async fn hartex_main() -> HarTexResult<()> {
    let span = tracing::info_span!("version info");
    span.in_scope(|| {
        tracing::info!("HarTex {HARTEX_BUILD}");
    });

    let span = tracing::trace_span!("initialize startup environment");
    let environment = span.in_scope(StartupEnv::get);

    let span = tracing::trace_span!("initialize database manipulation environment");
    span.in_scope(hartex_dbmani::init_env);

    let span = tracing::trace_span!("initialize plugin environment");
    span.in_scope(hartex_plugins::init_env);

    let span = tracing::trace_span!("pre-startup phase");
    let (cluster, http, events, cache) =
        pre_startup::pre_startup(environment).instrument(span).await;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let span = tracing::trace_span!("framework setup");
    let (emitter, framework_events) = span.in_scope(fw_setup::framework_setup);

    let span = tracing::trace_span!("ctrlc handler");
    span.in_scope(ctrlc::ctrlc_handler);

    let span = tracing::trace_span!("panic handler");
    span.in_scope(|| {
        tracing::trace!("registering panic handler");

        SyncLazy::force(&RUST_DEFAULT_PANIC_HOOK);
    });

    let mut events = events
        .map(Either::Left)
        .merge(framework_events.map(Either::Right));

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

/// # Function `report_ibe`
///
/// Reports an Internal Bot Error.
pub fn report_ibe(bug_report_url: &str) {
    tracing::error!("error: internal bot error: unexpected panic");
    tracing::error!("note: the bot unexpectedly panicked. this is a bug.");
    tracing::error!("note: we would appreciate a bug report: {bug_report_url}");
}
