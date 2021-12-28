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

use crate::{
    entities::guild::{
        role::RoleEntity,
        GuildEntity
    },
    entity::Entity,
    inmemory::InMemoryBackend,
    repositories::guild::{
        role::RoleRepository,
        GuildRepository
    },
    repository::{
        GetEntityFuture,
        Repository
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

    fn entity(&self, _: E::Id) -> GetEntityFuture<E, ()> {
        todo!()
    }
}

impl GuildRepository<InMemoryBackend> for InMemoryRepository<GuildEntity> {}

impl RoleRepository<InMemoryBackend> for InMemoryRepository<RoleEntity> {}

pub trait EntityExt: Clone + Entity {}

impl EntityExt for GuildEntity {}

impl EntityExt for RoleEntity {}
