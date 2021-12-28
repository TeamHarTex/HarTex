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

//! # The `inmemory` Module
//!
//! This module contains the in-memory backend for the cache.

use std::{
    marker::PhantomData,
    sync::Arc
};

use dashmap::DashMap;
use hartex_base::discord::model::id::{
    GuildId,
    RoleId
};

use crate::{
    backend::Backend,
    entities::guild::{
        role::RoleEntity,
        GuildEntity
    },
    entity::Entity,
    inmemory::repository::InMemoryRepository
};

pub mod repository;

/// # Struct `InMemoryBackend`
///
/// In-memory backend for the cache.
#[derive(Clone)]
pub struct InMemoryBackend(Arc<InMemoryBackendRef>);

impl InMemoryBackend {
    fn repository<T: Entity>(&self) -> InMemoryRepository<T> {
        InMemoryRepository(self.clone(), PhantomData)
    }
}

impl Backend for InMemoryBackend {
    type Error = ();
    type GuildRepository = InMemoryRepository<GuildEntity>;
    type RoleRepository = InMemoryRepository<RoleEntity>;

    fn guilds(&self) -> Self::GuildRepository {
        self.repository::<GuildEntity>()
    }

    fn roles(&self) -> Self::RoleRepository {
        self.repository::<RoleEntity>()
    }
}

struct InMemoryBackendRef {
    guilds: DashMap<GuildId, GuildEntity>,
    roles: DashMap<RoleId, RoleEntity>
}
