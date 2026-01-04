/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use tracing_core::{LevelFilter, Subscriber};
use tracing_error::ErrorLayer;
use tracing_subscriber::{Registry, filter::Targets, fmt::Layer, layer::SubscriberExt};

pub mod eyre;

#[must_use]
pub fn subscriber() -> impl Subscriber {
    let fmt_layer = Layer::default().with_target(true).with_level(true);
    let targets_layer = Targets::new()
        .with_default(LevelFilter::TRACE)
        .with_target("h2::client", LevelFilter::OFF)
        .with_target("h2::codec::framed_read", LevelFilter::OFF)
        .with_target("h2::codec::framed_write", LevelFilter::OFF)
        .with_target("h2::frame::headers", LevelFilter::OFF)
        .with_target("h2::frame::reset", LevelFilter::OFF)
        .with_target("h2::frame::settings", LevelFilter::OFF)
        .with_target("h2::frame::window_update", LevelFilter::OFF)
        .with_target("h2::hpack::decoder", LevelFilter::OFF)
        .with_target("h2::proto::connection", LevelFilter::OFF)
        .with_target("h2::proto::settings", LevelFilter::OFF)
        .with_target("h2::proto::streams::counts", LevelFilter::OFF)
        .with_target("h2::proto::streams::flow_control", LevelFilter::OFF)
        .with_target("h2::proto::streams::prioritize", LevelFilter::OFF)
        .with_target("h2::proto::streams::recv", LevelFilter::OFF)
        .with_target("h2::proto::streams::send", LevelFilter::OFF)
        .with_target("h2::proto::streams::state", LevelFilter::OFF)
        .with_target("h2::proto::streams::store", LevelFilter::OFF)
        .with_target("h2::proto::streams::stream", LevelFilter::OFF)
        .with_target("h2::proto::streams::streams", LevelFilter::OFF)
        .with_target("h2::server", LevelFilter::OFF)
        .with_target("hyper_util::client::legacy::client", LevelFilter::OFF)
        .with_target(
            "hyper_util::client::legacy::connect::http",
            LevelFilter::OFF,
        )
        .with_target("hyper_util::client::legacy::pool", LevelFilter::OFF)
        .with_target(
            "tonic::transport::channel::service::connection",
            LevelFilter::OFF,
        )
        .with_target(
            "tonic::transport::channel::service::reconnect",
            LevelFilter::OFF,
        )
        .with_target("tower::buffer::service", LevelFilter::OFF)
        .with_target("tower::buffer::worker", LevelFilter::OFF)
        .with_target("tower_http", LevelFilter::TRACE)
        .with_target("twilight_gateway::shard", LevelFilter::OFF)
        .with_target("twilight_http::client", LevelFilter::OFF);

    Registry::default()
        .with(fmt_layer)
        .with(targets_layer)
        .with(ErrorLayer::default())
}
