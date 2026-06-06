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

use std::sync::Arc;

use futures::StreamExt;
use kameo::{
    actor::{Actor, ActorRef, WeakActorRef},
    error::ActorStopReason,
    message::{Context, Message},
};
use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use tokio::sync::{RwLock, watch::Receiver};
use tracing::{Instrument, instrument};
use twilight_gateway::{Latency, Message as GatewayMessage, MessageSender, Shard as TwilightShard};
use twilight_model::gateway::{
    CloseFrame, event::GatewayEventDeserializer, payload::outgoing::RequestGuildMembers,
};

use crate::leader::{
    messages::{ShardLatency, ShardRequestGuildMembers},
    replies::ShardLatencyReply,
};

pub struct Shard {
    latency_lock: Arc<RwLock<Latency>>,
    sender: MessageSender,
    shard_id: u32,
}

impl Actor for Shard {
    type Args = (TwilightShard, Receiver<bool>);
    type Error = ();

    #[instrument(
        name = "shard_actor",
        skip(shard, receiver),
        fields(shard_id = shard.id().number())
    )]
    async fn on_start(
        (mut shard, receiver): Self::Args,
        _: ActorRef<Self>,
    ) -> Result<Self, Self::Error> {
        let id = shard.id();
        let id_num = id.number();

        let sender = shard.sender();

        let latency_lock = Arc::new(RwLock::new(shard.latency().clone()));
        let latency_cloned = latency_lock.clone();

        tokio::spawn(
            async move {
                let mut shutdown = receiver;

                loop {
                    tokio::select! {
                        _ = shutdown.changed() => {
                            if *shutdown.borrow() {
                                break;
                            }
                        }
                        Some(message) = shard.next() => {
                            let mut guard = latency_cloned.write().await;
                            *guard = shard.latency().clone();

                            match message {
                                Ok(GatewayMessage::Text(text)) => {
                                    let Some(deserializer) = GatewayEventDeserializer::from_json(text.as_str()) else {
                                        continue;
                                    };
                                    let mut json = Deserializer::from_slice(text.as_bytes());

                                    if let Ok(_) = deserializer.deserialize(&mut json) {
                                    }
                                }
                                Ok(GatewayMessage::Close(_)) => break,
                                _ => continue,
                            }
                        }
                    }
                }

                shard.close(CloseFrame::NORMAL);
            }.in_current_span(),
        );

        Ok(Self {
            latency_lock,
            sender,
            shard_id: id_num,
        })
    }

    #[instrument(name = "shard_actor", skip(self), fields(id = self.shard_id))]
    async fn on_stop(
        &mut self,
        _: WeakActorRef<Self>,
        _: ActorStopReason,
    ) -> Result<(), Self::Error> {
        tracing::warn!("shard stopping");
        Ok(())
    }
}

impl Message<ShardLatency> for Shard {
    type Reply = ShardLatencyReply;

    async fn handle(&mut self, _: ShardLatency, _: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let latency = self.latency_lock.read().await.clone();
        ShardLatencyReply { latency }
    }
}

impl Message<ShardRequestGuildMembers> for Shard {
    type Reply = ();

    async fn handle(
        &mut self,
        cmd: ShardRequestGuildMembers,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let command = RequestGuildMembers::builder(cmd.guild_id).query("", None);
        self.sender.command(&command).unwrap();
    }
}
