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

//! # The HarTex Discord Cache
//!
//! The implementation of a cache for storing Discord entities.

#![feature(once_cell)]

use std::lazy::SyncLazy;

use tokio::runtime::Runtime;

pub mod entities;
#[cfg(postgres)]
pub mod postgres;
#[cfg_attr(postgres, path = "postgres/repositories.rs")]
pub mod repositories;
pub mod update;

pub(in crate) static TOKIO_RT: SyncLazy<Runtime> = SyncLazy::new(|| {
    Runtime::new().unwrap()
});

pub struct DiscordCache;

impl DiscordCache {
    #[cfg(postgres)]
    pub fn update<'a>(
        &'a self,
        updatable: &'a impl update::CacheUpdatable<postgres::PostgresBackend>,
    ) -> Result<(), postgres::error::PostgresBackendError> {
        TOKIO_RT.block_on(async {
            updatable.update(&DiscordCache).await
        })
    }
}

pub fn init_tokio_rt() {
    SyncLazy::force(&TOKIO_RT);
}

#[cfg(not(any(postgres)))]
compile_error!("cache backend not specified; it is mandatory to specify the backend to use in the build configuration file: `buildconf.toml`");
