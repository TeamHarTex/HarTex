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

use futures::future;
use kameo::{
    actor::{Actor, ActorRef, Spawn, WeakActorRef},
    error::ActorStopReason,
    message::{Context, Message},
    remote::RemoteMessage,
    reply::ForwardedReply,
};
use tokio::sync::watch::{self, Sender};
use twilight_gateway::{Shard as TwilightShard, ShardId};

use crate::leader::{Shard, messages::ForwardToShard};

pub struct ShardManager {
    shards: HashMap<ShardId, (Sender<bool>, ActorRef<Shard>)>,
}

impl Actor for ShardManager {
    type Args = Vec<TwilightShard>;

    type Error = ();

    async fn on_start(args: Self::Args, _: ActorRef<Self>) -> Result<Self, Self::Error> {
        tracing::info!("starting {} shards...", args.len());

        let shards = args
            .into_iter()
            .map(|shard| {
                let (sender, receiver) = watch::channel(false);
                (shard.id(), (sender, Shard::spawn((shard, receiver))))
            })
            .collect::<HashMap<_, _>>();

        Ok(Self { shards })
    }

    async fn on_stop(
        &mut self,
        _: WeakActorRef<Self>,
        _: ActorStopReason,
    ) -> Result<(), Self::Error> {
        tracing::warn!("stopping {} shards...", self.shards.len());

        let futures = future::join_all(self.shards.values().map(|(sender, shard)| async {
            sender.send(true).unwrap();

            shard.stop_gracefully().await.ok();
            shard.wait_for_shutdown_result()
        }));

        futures.await;

        Ok(())
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
        let (_, shard_ref) = self.shards.get(&msg.id).unwrap();
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
