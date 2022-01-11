/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
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

//! # The `update` Module
//!
//! This module implements the `CacheUpdate` trait for various Discord events.

use futures_util::{
    future::FutureExt,
    stream::{
        FuturesUnordered,
        TryStreamExt
    }
};
use hartex_base::discord::model::{
    channel::Channel,
    gateway::payload::incoming::ChannelCreate
};
use hartex_cache_base::{
    repository::Repository,
    UpdateCacheFuture
};

use crate::{
    backend::DiscordBackend,
    cache::DiscordCache,
    entities::{
        channel::ChannelEntity,
        user::UserEntity
    }
};

/// # Trait `CacheUpdate`
///
/// A trait for callbacks / events that may update the cache.
pub trait DiscordCacheUpdate<B: DiscordBackend> {
    fn update<'a>(&'a self, cache: &'a DiscordCache<B>) -> UpdateCacheFuture<'a, B>;
}

impl<B: DiscordBackend> DiscordCacheUpdate<B> for ChannelCreate {
    fn update<'a>(&'a self, cache: &'a DiscordCache<B>) -> UpdateCacheFuture<'a, B> {
        match &self.0 {
            Channel::Group(group) => {
                let futures = FuturesUnordered::new();

                futures.push(
                    cache.users
                        .upsert_many(group.recipients.iter().cloned().map(UserEntity::from))
                );

                let entity = ChannelEntity::from(group.clone());

                futures.push(cache.channels.upsert(entity));

                futures.try_collect().boxed()
            }
            _ => todo!()
        }
    }
}
