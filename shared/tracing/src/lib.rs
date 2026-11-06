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

use tracing_core::{LevelFilter, Subscriber};
use tracing_error::ErrorLayer;
use tracing_subscriber::{Registry, filter::Targets, fmt::Layer as FmtLayer, layer::SubscriberExt};

pub fn subscriber() -> impl Subscriber {
    let fmt = FmtLayer::default().with_target(true).with_level(true);
    let targets = Targets::new().with_default(LevelFilter::TRACE);
    let error = ErrorLayer::default();

    Registry::default().with(fmt).with(targets).with(error)
}
