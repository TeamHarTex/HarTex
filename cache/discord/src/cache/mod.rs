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

//! # The `cache` Module
//!
//! This module contains a base cache structure that supports various backends.

use std::sync::Arc;

use hartex_cache_base::Cache;

use crate::backend::DiscordBackend;

pub mod update;

/// # Struct `DiscordCache`
///
/// A Discord cache.
#[allow(clippy::module_name_repetitions)]
pub struct DiscordCache<B: DiscordBackend> {
    backend: Arc<B>,
    pub channels: B::ChannelRepository,
    pub users: B::UserRepository
}

impl<B: DiscordBackend> DiscordCache<B> {
    /// # Static Method `with_backend`
    ///
    /// Creates a new instance of a cache with the provided cache backend.
    pub fn with_backend(backend: impl Into<Arc<B>>) -> Self {
        let backend = backend.into();
        let channels = backend.channels();
        let users = backend.users();

        Self {
            backend,
            channels,
            users
        }
    }
}

impl<B: DiscordBackend> Cache<B> for DiscordCache<B> {
    fn backend(&self) -> &Arc<B> {
        &self.backend
    }
}
