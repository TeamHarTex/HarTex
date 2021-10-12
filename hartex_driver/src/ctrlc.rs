//! # The `ctrlc` Module
//!
//! This module provides the setting of the ctrlc handler for the bot.

use std::process;

use hartex_core::{
    ctrlc,
    logging::tracing
};

/// # Function `ctrlc_handler`
///
/// Sets the ctrl+c handler.
pub fn ctrlc_handler() {
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
}
