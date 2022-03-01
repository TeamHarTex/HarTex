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

use cache_base::Cache;

pub mod entities;
#[cfg(postgres)]
pub mod postgres;
#[cfg_attr(postgres, path = "postgres/repositories.rs")]
pub mod repositories;

#[cfg(postgres)]
pub type PostgresCache = Cache<postgres::PostgresBackend>;

#[cfg(not(any(postgres)))]
compile_error!("cache backend not specified; it is mandatory to specify the backend to use in the build configuration file: `buildconf.toml`");
