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

use std::collections::HashMap;

use kameo::{
    actor::{Actor, ActorRef, Spawn},
    message::{Context, Message},
    remote::RemoteMessage,
    reply::ForwardedReply,
};
use twilight_gateway::{Shard as TwilightShard, ShardId};

use crate::leader::{Shard, messages::ForwardToShard};

pub struct ShardManager {
    shards: HashMap<ShardId, ActorRef<Shard>>,
}

impl Actor for ShardManager {
    type Args = Vec<TwilightShard>;

    type Error = ();

    async fn on_start(args: Self::Args, _: ActorRef<Self>) -> Result<Self, Self::Error> {
        let shards = args
            .into_iter()
            .map(|shard| (shard.id(), Shard::spawn(shard)))
            .collect::<HashMap<_, _>>();

        Ok(Self { shards })
    }
}

impl<M> Message<ForwardToShard<M>> for ShardManager
where
    Shard: Message<M>,
    M: Send + 'static,
{
    type Reply = ForwardedReply<M, <Shard as Message<M>>::Reply>;

    async fn handle(
        &mut self,
        msg: ForwardToShard<M>,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let shard_ref = self.shards.get(&msg.id).unwrap();
        ctx.forward(shard_ref, msg.message).await
    }
}

impl<M> RemoteMessage<ForwardToShard<M>> for ShardManager
where
    Shard: RemoteMessage<M>,
    M: Send + 'static,
{
    const REMOTE_ID: &'static str = "SHARD_MANAGER_FORWARD_TO_SHARD";
}
