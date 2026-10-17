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

use std::time::Duration;

use async_nats::jetstream::{Context, stream::{Config, RetentionPolicy, StorageType}};

use crate::error::GatewayResult;

const STREAM_NAME: &str = "gateway-nats-events";

pub async fn ensure_stream(jetstream: &Context) -> GatewayResult<()> {
    jetstream.get_or_create_stream(Config {
        name: STREAM_NAME.to_owned(),
        subjects: vec!["gateway.events.>".to_owned()],
        retention: RetentionPolicy::Limits,
        storage: StorageType::File,
        max_age: Duration::from_secs(60 * 60 * 24),
        ..Default::default()
    }).await.into()
}

pub async fn publish_event(_: &Context) -> GatewayResult<()> {
    todo!()
}
