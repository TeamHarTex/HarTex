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

//! An interface for manipulating the `PostgreSQL` databases used by `HarTex` Discord bot.

#![allow(non_snake_case)]
#![deny(clippy::pedantic, warnings)]
#![forbid(unsafe_code)]
#![feature(once_cell)]

use std::{future::Future, lazy::SyncLazy, pin::Pin};

use hartex_base::error::HarTexResult;
use hartex_env::DatabaseEnv;

pub mod guildconf;
pub mod whitelist;

/// Useful environment variables for database manipulation
pub static DATABASE_ENV: SyncLazy<DatabaseEnv> = SyncLazy::new(DatabaseEnv::get);

///  A pending future that is yet to return.
type PendingFuture<T> = Pin<Box<dyn Future<Output = HarTexResult<T>> + Send>>;

/// Initialize the environment variables for later use, must be called in the "entry point" in
/// the `hartex_driver` crate for the environment variables to be usable.
pub fn init_env() {
    SyncLazy::force(&DATABASE_ENV);
}
