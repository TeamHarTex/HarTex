/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

use rdkafka::ClientConfig;

use crate::serde::Serializer;
use crate::types::CompressionType;

pub trait ClientConfigUtils {
    fn bootstrap_servers(&mut self, servers: impl Iterator<Item = String>) -> &mut Self;

    fn compression_type(&mut self, compression: CompressionType) -> &mut Self;

    fn delivery_timeout_ms(&mut self, timeout: u32) -> &mut Self;

    fn key_serializer(&mut self, serializer: impl Serializer) -> &mut Self;

    fn value_serializer(&mut self, serializer: impl Serializer) -> &mut Self;
}

impl ClientConfigUtils for ClientConfig {
    fn bootstrap_servers(&mut self, servers: impl Iterator<Item = String>) -> &mut Self {
        self.set(
            "bootstrap.servers",
            servers.intersperse(String::from(";")).collect::<String>(),
        )
    }

    fn compression_type(&mut self, compression: CompressionType) -> &mut Self {
        self.set("compression.type", compression)
    }

    fn delivery_timeout_ms(&mut self, timeout: u32) -> &mut Self {
        self.set("delivery.timeout.ms", timeout.to_string())
    }

    fn key_serializer(&mut self, serializer: impl Serializer) -> &mut Self {
        self.set("key.serializer", serializer.java_fqn())
    }

    fn value_serializer(&mut self, serializer: impl Serializer) -> &mut Self {
        self.set("value.serializer", serializer.java_fqn())
    }
}
