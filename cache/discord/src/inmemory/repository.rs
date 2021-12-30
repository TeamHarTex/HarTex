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
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `repository` Module
//!
//! This module implements a repository with an in-memory backend

use std::{
    marker::PhantomData,
    sync::Mutex
};

use dashmap::DashMap;
use futures_util::{
    future::{
        self,
        FutureExt
    },
    stream::{
        self,
        StreamExt
    }
};
use hartex_base::discord::model::id::{
    EmojiId,
    GuildId,
    RoleId,
    UserId
};

use crate::{
    entities::{
        channel::attachment::AttachmentEntity,
        guild::{
            emoji::EmojiEntity,
            member::MemberEntity,
            role::RoleEntity,
            GuildEntity
        },
        user::{
            current_user::CurrentUserEntity,
            UserEntity
        }
    },
    entity::Entity,
    inmemory::{
        InMemoryBackend,
        InMemoryBackendError
    },
    repositories::{
        channel::attachment::AttachmentRepository,
        guild::{
            emoji::EmojiRepository,
            member::MemberRepository,
            role::RoleRepository,
            GuildRepository
        },
        user::{
            current_user::CurrentUserRepository,
            UserRepository
        }
    },
    repository::{
        GetEntityFuture,
        Repository,
        SingleEntityRepository,
        StreamEntitiesFuture,
        StreamEntityIdsFuture
    }
};

/// # Struct `InMemoryRepository`
///
/// A cache repository with the in-memory cache backend.
#[allow(clippy::module_name_repetitions)]
pub struct InMemoryRepository<T>(pub(crate) InMemoryBackend, pub(crate) PhantomData<T>);

impl<E: EntityExt> Repository<E, InMemoryBackend> for InMemoryRepository<E> {
    fn backend(&self) -> InMemoryBackend {
        self.0.clone()
    }

    fn entity(&self, entity_id: E::Id) -> GetEntityFuture<E, InMemoryBackendError> {
        future::ok(
            E::repository(&self.0)
                .get(&entity_id)
                .map(|entry| entry.value().clone())
        )
        .boxed()
    }
}

impl AttachmentRepository<InMemoryBackend> for InMemoryRepository<AttachmentEntity> {}

impl CurrentUserRepository<InMemoryBackend> for InMemoryRepository<CurrentUserEntity> {}

impl EmojiRepository<InMemoryBackend> for InMemoryRepository<EmojiEntity> {}

impl GuildRepository<InMemoryBackend> for InMemoryRepository<GuildEntity> {
    fn emoji_ids(
        &self,
        guild_id: GuildId
    ) -> StreamEntityIdsFuture<'_, EmojiId, InMemoryBackendError> {
        let stream = (self.0).0.guild_emojis.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|id| Ok(*id)).collect::<Vec<_>>()).boxed()
        );

        future::ok(stream).boxed()
    }

    fn emojis(
        &self,
        guild_id: GuildId
    ) -> StreamEntitiesFuture<'_, EmojiEntity, InMemoryBackendError> {
        let emoji_ids = match (self.0).0.guild_emojis.get(&guild_id) {
            Some(ids) => ids.clone(),
            None => return future::ok(stream::empty().boxed()).boxed()
        };

        let iter = emoji_ids.into_iter().filter_map(move |emoji_id| {
            (self.0)
                .0
                .emojis
                .get(&emoji_id)
                .map(|entry| Ok(entry.value().clone()))
        });
        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn member_user_ids(
        &self,
        guild_id: GuildId
    ) -> StreamEntityIdsFuture<'_, UserId, InMemoryBackendError> {
        let stream = (self.0).0.guild_members.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|id| Ok(*id)).collect::<Vec<_>>()).boxed()
        );

        future::ok(stream).boxed()
    }

    fn members(
        &self,
        guild_id: GuildId
    ) -> StreamEntitiesFuture<'_, MemberEntity, InMemoryBackendError> {
        let member_user_ids = match (self.0).0.guild_members.get(&guild_id) {
            Some(ids) => ids.clone(),
            None => return future::ok(stream::empty().boxed()).boxed()
        };

        let iter = member_user_ids
            .into_iter()
            .filter_map(move |member_user_id| {
                (self.0)
                    .0
                    .members
                    .get(&(guild_id, member_user_id))
                    .map(|entry| Ok(entry.value().clone()))
            });
        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn role_ids(
        &self,
        guild_id: GuildId
    ) -> StreamEntityIdsFuture<'_, RoleId, InMemoryBackendError> {
        let stream = (self.0).0.guild_roles.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|id| Ok(*id)).collect::<Vec<_>>()).boxed()
        );

        future::ok(stream).boxed()
    }

    fn roles(
        &self,
        guild_id: GuildId
    ) -> StreamEntitiesFuture<'_, RoleEntity, InMemoryBackendError> {
        let role_ids = match (self.0).0.guild_roles.get(&guild_id) {
            Some(ids) => ids.clone(),
            None => return future::ok(stream::empty().boxed()).boxed()
        };

        let iter = role_ids.into_iter().filter_map(move |role_id| {
            (self.0)
                .0
                .roles
                .get(&role_id)
                .map(|entry| Ok(entry.value().clone()))
        });
        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }
}

impl MemberRepository<InMemoryBackend> for InMemoryRepository<MemberEntity> {
    fn roles(
        &self,
        guild_id: GuildId,
        user_id: UserId
    ) -> StreamEntitiesFuture<'_, RoleEntity, InMemoryBackendError> {
        let role_ids = match (self.0).0.members.get(&(guild_id, user_id)) {
            Some(member) => member.role_ids(),
            None => return future::ok(stream::empty().boxed()).boxed()
        };

        let iter = role_ids.into_iter().filter_map(move |role_id| {
            (self.0)
                .0
                .roles
                .get(&role_id)
                .map(|entry| Ok(entry.value().clone()))
        });
        let stream = stream::iter(iter);

        future::ok(stream.boxed()).boxed()
    }
}

impl RoleRepository<InMemoryBackend> for InMemoryRepository<RoleEntity> {}

impl UserRepository<InMemoryBackend> for InMemoryRepository<UserEntity> {}

impl SingleEntityRepository<CurrentUserEntity, InMemoryBackend>
    for InMemoryRepository<CurrentUserEntity>
{
    fn backend(&self) -> InMemoryBackend {
        self.0.clone()
    }

    fn entity(&self) -> GetEntityFuture<CurrentUserEntity, InMemoryBackendError> {
        future::ok(
            CurrentUserEntity::lock(&self.0)
                .lock()
                .expect("current user mutex is poisoned")
                .clone()
        )
        .boxed()
    }
}

pub trait EntityExt: Clone + Entity {
    /// # Trait Method `repository`
    ///
    /// Returns the corresponding repository of the entity.
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>;
}

impl EntityExt for AttachmentEntity {
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self> {
        &backend.0.attachments
    }
}

impl EntityExt for EmojiEntity {
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self> {
        &backend.0.emojis
    }
}

impl EntityExt for GuildEntity {
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self> {
        &backend.0.guilds
    }
}

impl EntityExt for MemberEntity {
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self> {
        &backend.0.members
    }
}

impl EntityExt for RoleEntity {
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self> {
        &backend.0.roles
    }
}

impl EntityExt for UserEntity {
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self> {
        &backend.0.users
    }
}

pub trait SingleEntityExt: Clone + Entity {
    fn lock(backend: &InMemoryBackend) -> &Mutex<Option<Self>>;
}

impl SingleEntityExt for CurrentUserEntity {
    fn lock(backend: &InMemoryBackend) -> &Mutex<Option<Self>> {
        &backend.0.current_user
    }
}
