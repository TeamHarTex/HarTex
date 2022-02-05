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

//! Implementations of the `CacheUpdate` trait for various Discord events.

use futures_util::{
    future::FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use hartex_base::discord::model::{
    channel::{Channel, GuildChannel},
    gateway::payload::incoming::ChannelCreate,
};
use hartex_cache_base::{repository::Repository, UpdateCacheFuture};

use crate::{
    backend::DiscordBackend,
    cache::DiscordCache,
    entities::{
        channel::{thread::ThreadEntity, ChannelEntity},
        user::UserEntity,
    },
};

/// A trait for callbacks / events that may update the cache.
#[allow(clippy::module_name_repetitions)]
pub trait DiscordCacheUpdate<B: DiscordBackend> {
    fn update<'a>(&'a self, cache: &'a DiscordCache<B>) -> UpdateCacheFuture<'a, B>;
}

impl<B: DiscordBackend> DiscordCacheUpdate<B> for ChannelCreate {
    fn update<'a>(&'a self, cache: &'a DiscordCache<B>) -> UpdateCacheFuture<'a, B> {
        match &self.0 {
            Channel::Group(group) => {
                let futures = FuturesUnordered::new();

                futures.push(
                    cache
                        .users
                        .upsert_many(group.recipients.iter().cloned().map(UserEntity::from)),
                );

                let entity = ChannelEntity::from(group.clone());

                futures.push(cache.channels.upsert(entity));

                futures.try_collect().boxed()
            }
            Channel::Guild(GuildChannel::Category(category)) => {
                let entity = ChannelEntity::from(category.clone());

                cache.channels.upsert(entity)
            }
            Channel::Guild(GuildChannel::NewsThread(news)) => {
                let entity = ThreadEntity::from(news.clone());

                cache.threads.upsert(entity)
            }
            Channel::Guild(GuildChannel::PrivateThread(private)) => {
                let entity = ThreadEntity::from(private.clone());

                cache.threads.upsert(entity)
            }
            Channel::Guild(GuildChannel::PublicThread(public)) => {
                let entity = ThreadEntity::from(public.clone());

                cache.threads.upsert(entity)
            }
            Channel::Guild(GuildChannel::Stage(stage)) => {
                let entity = ChannelEntity::from(stage.clone());

                cache.channels.upsert(entity)
            }
            Channel::Guild(GuildChannel::Text(text)) => {
                let entity = ChannelEntity::from(text.clone());

                cache.channels.upsert(entity)
            }
            Channel::Guild(GuildChannel::Voice(voice)) => {
                let entity = ChannelEntity::from(voice.clone());

                cache.channels.upsert(entity)
            }
            Channel::Private(private) => {
                let entity = ChannelEntity::from(private.clone());

                cache.channels.upsert(entity)
            }
        }
    }
}
