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
//! This module contains a base repository trait for different cache repositories.

use std::{
    future::Future,
    pin::Pin
};

use crate::{
    backend::Backend,
    entity::Entity
};

/// # Trait `Repository`
///
/// A repository for a specific entity in a specific backend.
pub trait Repository<E: Entity, B: Backend> {
    /// # Trait Method `backend`
    ///
    /// Returns an immutable reference to the backend of this repository.
    fn backend(&self) -> B;

    /// # Trait Method `entity`
    ///
    /// Retrieves an entity from the cache by its id.
    fn entity(&self, id: E::Id) -> GetEntityFuture<E, B::Error>;
}

/// # Typealias `GetEntityFuture`
///
/// Type alias for a future to retrieve an entity from the cache.
pub type GetEntityFuture<'a, T, E> =
    Pin<Box<dyn Future<Output = Result<Option<T>, E>> + Send + 'a>>;
