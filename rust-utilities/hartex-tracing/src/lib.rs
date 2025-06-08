/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Logging Facilities

pub use formati::debug;
pub use formati::format;
pub use formati::error;
pub use formati::info;
pub use formati::trace;
pub use formati::warn;

use tracing_core::LevelFilter;
use tracing_core::Subscriber;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

/// Create a new `tracing` subscriber.
pub fn subscriber() -> impl Subscriber {
    let fmt_layer = Layer::default()
        .pretty()
        .with_timer(OffsetTime::local_rfc_3339().unwrap())
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true);
    let targets_layer = Targets::new()
        .with_default(LevelFilter::TRACE)
        .with_target("hyper_util::client::legacy::client", LevelFilter::OFF)
        .with_target("hyper_util::client::legacy::connect::http", LevelFilter::OFF)
        .with_target("hyper_util::client::legacy::pool", LevelFilter::OFF)
        .with_target("tower_http", LevelFilter::TRACE)
        .with_target("twilight_gateway::shard", LevelFilter::OFF)
        .with_target("twilight_http::client", LevelFilter::OFF);

    Registry::default()
        .with(fmt_layer)
        .with(targets_layer)
}
