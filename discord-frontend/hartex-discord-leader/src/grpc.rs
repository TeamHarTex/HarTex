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

use futures_util::StreamExt as FutureStreamExt;
use hartex_discord_core::discord::gateway::Message as GatewayMessage;
use hartex_discord_core::discord::gateway::Shard;
use hartex_discord_core::discord::gateway::queue::Queue;
use hartex_discord_core::tokio;
use hartex_discord_grpc_protos::gateway::gateway_client::GatewayClient;
use tonic::transport::Channel;

/// Handle inbound AND outbound messages for a given shard.
pub async fn handle<Q>(
    shard: &mut Shard<Q>,
    client: GatewayClient<Channel>
) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    // let shard_id = shard.id().number();
    // let sender = shard.sender();
    tokio::select! {
        _ = inbound(shard, client) => {},
        // _ = outbound((shard_id, sender), consumer) => {}
    }

    Ok(())
}

/// Handle inbound traffic.
#[allow(clippy::match_wildcard_for_single_variants)]
async fn inbound<Q>(shard: &mut Shard<Q>, _: GatewayClient<Channel>) -> miette::Result<()>
where
    Q: Queue + Send + Sync + Sized + Unpin + 'static,
{
    while let Some(result) = shard.next().await {
        match result {
            Ok(message) => {
                let Some(_) = (match message {
                    // todo: handle close frame
                    GatewayMessage::Text(string) => Some(string.into_bytes()),
                    _ => None,
                }) else {
                    continue;
                };

                hartex_tracing::trace!(
                    "[shard {shard.id().number()}] received payload from gateway",
                );

                // send payload to worker process
            }
            Err(error) => {
                hartex_tracing::warn!(
                    "[shard {shard.id()}] error when receiving gateway message: {error}",
                );
            }
        }
    }

    Ok(())
}

/*/// Handle outbound traffic.
async fn outbound(
    (shard_id, sender): (u32, MessageSender),
    consumer: Arc<StreamConsumer>,
) -> miette::Result<()> {
    while let Some(result) = consumer.stream().next().await {
        let Ok(message) = result else {
            let error = result.unwrap_err();
            println!("{:?}", Err::<(), KafkaError>(error).into_diagnostic());

            continue;
        };

        let key = str::from_utf8(message.key().unwrap()).unwrap();

        if key.contains("REQUEST_GUILD_MEMBERS") {
            let bytes = message.payload().unwrap();

            let command = serde_json::from_slice::<RequestGuildMembers>(bytes).into_diagnostic()?;
            let scanned: u32 =
                scan!("OUTBOUND_REQUEST_GUILD_MEMBERS_{}" <- key).into_diagnostic()?;

            if shard_id != scanned {
                continue;
            }

            sender.command(&command).into_diagnostic()?;
        }
    }

    Ok(())
}*/
