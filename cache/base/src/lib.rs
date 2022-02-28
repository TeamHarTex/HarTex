/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

//! # The HarTex Cache Framework
//!
//! The `cache_base` crate provides a foundation of cache implementations for HarTex.
//!
//! The cache can be built upon any backend, examples of supported backends are redis,
//! in-memory, and mostly any relational databases.
//!
//! The cache of HarTex is specifically using a PostgreSQL database for its caching
//! implementation.

use std::hash::Hash;

pub mod future;

/// A cache.
pub struct Cache<B: Backend>;

/// A cache backend.
pub trait Backend: Send + Sized + Sync + 'static {
    /// Error returned by backend operations.
    type Error: Send + 'static;
}

/// An entity in the cache.
pub trait Entity: Send + Sync {
    /// An ID that uniquely identifies the entity.
    type Id: Copy + Eq + Hash + Send + Sync;

    fn id(&self) -> Self::Id;
}

/// A repository of a specific entity in the cache.
pub trait Repository<B: Backend, T: Entity> {
    /// Upsert an entity into the repository.
    fn upsert(&self, entity: T) -> future::UpsertEntityFuture<'_, B::Error>;
}
