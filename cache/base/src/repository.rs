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

//! # The `repository` Module
//!
//! This module contains a base repository trait for different cache repositories.

use std::{
    future::Future,
    pin::Pin
};

use futures_util::stream::Stream;

use crate::{
    backend::Backend,
    entity::Entity
};

/// # Trait `Repository`
///
/// A repository for a series of specific entities in a specific backend.
pub trait Repository<E: Entity, B: Backend> {
    /// # Trait Method `backend`
    ///
    /// Returns an immutable reference to the backend of this repository.
    fn backend(&self) -> B;

    /// # Trait Method `entity`
    ///
    /// Retrieves an entity from the cache by its id.
    fn entity(&self, id: E::Id) -> GetEntityFuture<E, B::Error>;

    /// # Trait Method `upsert`
    ///
    /// Upserts an entity into the cache.
    fn upsert(&self, entity: E) -> UpsertEntityFuture<'_, B::Error>;
}

/// # Trait `SingleEntityRepository`
///
/// A repository for a specific entity in a specific backend.
#[allow(clippy::module_name_repetitions)]
pub trait SingleEntityRepository<E: Entity, B: Backend> {
    /// # Trait Method `backend`
    ///
    /// Returns an immutable reference to the backend of this repository.
    fn backend(&self) -> B;

    /// # Trait Method `entity`
    ///
    /// Retrieves the underlying entity.
    fn entity(&self) -> GetEntityFuture<E, B::Error>;
}

/// # Typealias `GetEntityFuture`
///
/// Typealias for a future to retrieve an entity from the cache.
pub type GetEntityFuture<'a, T, E> =
Pin<Box<dyn Future<Output = Result<Option<T>, E>> + Send + 'a>>;

/// # Typealias `EntityStream`
///
/// Typealias for a stream of entities.
pub type EntityStream<'a, T, E> = Pin<Box<dyn Stream<Item = Result<T, E>> + Send + 'a>>;

/// # Typealias `StreamEntitiesFuture`
///
/// Typealias for a future to stream (retrieve a series of) entities from the cache.
pub type StreamEntitiesFuture<'a, T, E> =
Pin<Box<dyn Future<Output = Result<EntityStream<'a, T, E>, E>> + Send + 'a>>;

/// # Typealias `EntityIdStream`
///
/// Typealias for a stream of entity ids.
pub type EntityIdStream<'a, Id, E> = Pin<Box<dyn Stream<Item = Result<Id, E>> + Send + 'a>>;

/// # Typealias `StreamEntityIdsFuture`
///
/// Typealias for a future to stream (retrieve a series of) entities from the cache.
pub type StreamEntityIdsFuture<'a, Id, E> =
Pin<Box<dyn Future<Output = Result<EntityIdStream<'a, Id, E>, E>> + Send + 'a>>;

/// # Typealias `UpsertEntityFuture`
///
/// Typealias for a future to upsert an entity into the cache.
pub type UpsertEntityFuture<'a, E> = Pin<Box<dyn Future<Output = Result<(), E>> + Send + 'a>>;
