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

use std::marker::PhantomData;

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
use hartex_base::discord::model::id::{GuildId, RoleId, UserId};

use crate::{
    entities::guild::{
        member::MemberEntity,
        role::RoleEntity,
        GuildEntity
    },
    entity::Entity,
    inmemory::{
        InMemoryBackend,
        InMemoryBackendError
    },
    repositories::guild::{
        member::MemberRepository,
        role::RoleRepository,
        GuildRepository
    },
    repository::{
        GetEntityFuture,
        Repository,
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

    fn entity(&self, _: E::Id) -> GetEntityFuture<E, InMemoryBackendError> {
        todo!()
    }
}

impl GuildRepository<InMemoryBackend> for InMemoryRepository<GuildEntity> {
    fn member_user_ids(
        &self,
        guild_id: GuildId
    ) -> StreamEntityIdsFuture<UserId, InMemoryBackendError> {
        let stream = (self.0).0.guild_members.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed()
        );

        future::ok(stream).boxed()
    }

    fn members(
        &self,
        guild_id: GuildId
    ) -> StreamEntitiesFuture<MemberEntity, InMemoryBackendError> {
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
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed()
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

impl MemberRepository<InMemoryBackend> for InMemoryRepository<MemberEntity> {}

impl RoleRepository<InMemoryBackend> for InMemoryRepository<RoleEntity> {}

pub trait EntityExt: Clone + Entity {
    /// # Trait Method `repository`
    ///
    /// Returns the corresponding repository of the entity.
    fn repository(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>;
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
