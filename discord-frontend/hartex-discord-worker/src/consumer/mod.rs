/* SPDX-License-Identifier: AGPL-3.0-only
*
* This file is part of HarTex.
*
* HarTex
* Copyright (c) 2021-2022 HarTex Project Developers
*
* HarTex is free software; you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation; either version 3 of the License, or
* (at your option) any later version.
*
* HarTex is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License along
* with HarTex. If not, see <https://www.gnu.org/licenses/>.
*/

use std::str;

use futures_util::StreamExt;
use hartex_discord_core::discord::model::gateway::event::GatewayEventDeserializer;
use hartex_discord_core::log;
use lapin::options::BasicAckOptions;
use lapin::Consumer;
use serde::de::DeserializeSeed;
use serde_scan::scan;

use crate::consumer::error::{ConsumerError, ConsumerErrorKind};

mod entitycache;
mod error;

pub async fn consume(mut consumer: Consumer) -> hartex_discord_eyre::Result<()> {
    while let Some(result) = consumer.next().await {
        if let Ok(delivery) = result {
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("failed to ack");
            let value = delivery.routing_key.as_str();
            let scanned: (u8, u8) = scan!("CLUSTER {} SHARD {} PAYLOAD" <- value)?;

            let (gateway_deserializer, mut json_deserializer) = {
                let gateway_deserializer = GatewayEventDeserializer::from_json(
                    str::from_utf8(&delivery.data).map_err(|_| ConsumerError {
                        kind: ConsumerErrorKind::GatewayPayloadNotUTF8,
                        source: None,
                    })?,
                )
                .ok_or(ConsumerError {
                    kind: ConsumerErrorKind::InvalidGatewayPayload,
                    source: None,
                })?;

                let json_deserializer = serde_json::Deserializer::from_slice(&delivery.data);

                (gateway_deserializer, json_deserializer)
            };

            let event = gateway_deserializer
                .clone()
                .deserialize(&mut json_deserializer)
                .map_err(|source| ConsumerError {
                    kind: ConsumerErrorKind::DeserializationFailed,
                    source: Some(Box::new(source)),
                })?;

            log::trace!(
                "[cluster {} - shard {}] received {} event",
                scanned.0,
                scanned.1,
                gateway_deserializer.event_type_ref().unwrap_or("UNKNOWN")
            );
            entitycache::update_entitycache(&event).await?;
        }
    }

    Ok(())
}
