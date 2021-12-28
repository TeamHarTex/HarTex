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

//! # The `backend` Module
//!
//! This module contains a base backend trait for different cache backends.

use crate::repositories::guild::{
    role::RoleRepository,
    GuildRepository
};

/// # Trait `Backend`
///
/// A base trait for different cache backends (in-memory, database, redis, etc).
pub trait Backend: Send + Sized + Sync + 'static {
    /// # Typealias `Error`
    ///
    /// The backend error type.
    type Error: Send + 'static;

    /// # Typealias `GuildRepository`
    ///
    /// The repository for guild entities.
    type GuildRepository: GuildRepository<Self> + Send + Sync;

    /// # Typealias `RoleRepository`
    ///
    /// The repository for guild roles entities.
    type RoleRepository: RoleRepository<Self> + Send + Sync;

    /// # Trait Method `guilds`
    ///
    /// Returns the guild repository of the cache.
    fn guilds(&self) -> Self::GuildRepository;

    /// # Trait Method `roles`
    ///
    /// Returns the role repository of the cache.
    fn roles(&self) -> Self::RoleRepository;
}
