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

use std::sync::Arc;

use kameo::{
    Actor,
    actor::ActorRef,
    message::{Context, Message},
    prelude::{RemoteActor, RemoteMessage},
};
use tokio::sync::Mutex;
use twilight_gateway::{EventTypeFlags, MessageSender, Shard as TwilightShard, StreamExt};
use twilight_model::gateway::payload::outgoing::RequestGuildMembers;
use crate::leader::{
    messages::{ShardLatency, ShardRequestGuildMembers},
    replies::ShardLatencyReply,
};

pub struct Shard {
    sender: MessageSender,
    shard: Arc<Mutex<TwilightShard>>,
}

impl Actor for Shard {
    type Args = TwilightShard;
    type Error = ();

    async fn on_start(shard: Self::Args, _: ActorRef<Self>) -> Result<Self, Self::Error> {
        // todo: logging
        let sender = shard.sender();
        let shard_arc = Arc::new(Mutex::new(shard));

        let shard_cloned = shard_arc.clone();

        tokio::spawn(async move {
            while let Some(_) = shard_cloned
                .lock()
                .await
                .next_event(EventTypeFlags::all())
                .await
            {
                todo!()
            }
        });

        Ok(Self {
            sender,
            shard: shard_arc,
        })
    }
}

impl Message<ShardLatency> for Shard {
    type Reply = ShardLatencyReply;

    async fn handle(&mut self, _: ShardLatency, _: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let latency = self.shard.lock().await.latency().clone();
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
        let command = RequestGuildMembers::builder(cmd.guild_id)
            .query("", None);

        self.sender.command(&command).unwrap();
    }
}

impl RemoteActor for Shard {
    const REMOTE_ID: &'static str = "SHARD_ACTOR";
}

impl RemoteMessage<ShardLatency> for Shard {
    const REMOTE_ID: &'static str = "SHARD_LATENCY_MESSAGE";
}

impl RemoteMessage<ShardRequestGuildMembers> for Shard {
    const REMOTE_ID: &'static str = "SHARD_REQUEST_GUILD_MEMBERS";
}
