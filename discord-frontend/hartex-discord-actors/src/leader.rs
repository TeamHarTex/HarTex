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

use std::sync::{Arc, Mutex};

use kameo::{Actor, actor::ActorRef};
use twilight_gateway::{EventTypeFlags, Shard, StreamExt};

pub struct ShardActor {
    shard: Arc<Mutex<Shard>>,
}

impl Actor for ShardActor {
    type Args = Shard;
    type Error = ();

    async fn on_start(shard: Self::Args, _: ActorRef<Self>) -> Result<Self, Self::Error> {
        let shardref = Arc::new(Mutex::new(shard));
        let cloned = shardref.clone();

        tokio::spawn(async move {
            let mut lock = cloned.lock().unwrap();

            while let Some(_) = lock.next_event(EventTypeFlags::all()).await {

            }
        });

        Ok(Self {
            shard: shardref,
        })
    }
}
