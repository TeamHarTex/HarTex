/* SPDX-License-Identifier: GPL-2.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

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
#[allow(clippy::module_name_repetitions)]
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
