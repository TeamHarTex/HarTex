/*
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # `hartex_utils` - The `HarTex` Utilities Library
//!
//! The `hartex_utils` library provides several utilities for the `HarTex` Discord bot.

#![deny(clippy::pedantic, warnings, unsafe_code)]

use std::{
    future::Future,
    pin::Pin
};

use hartex_core::error::HarTexResult;

pub mod cdn;
pub mod result;

/// # Constant Function `shard_id`
///
/// Computes the shard id for a specific guild by the guild id and the number of shards.
///
/// ## Parameters
/// - `guild_id`, type `u64`: the guild id
/// - `shards`, type `u64`: the total number of shards
#[must_use]
pub const fn shard_id(guild_id: u64, shards: u64) -> u64 {
    (guild_id >> 22) % shards
}

/// # Typealias `FutureRetType`
///
/// A typealias for `Pin<Box<dyn Future<Output = HarTexResult<T>> + Send + 'a>>`.
pub type FutureRetType<'a, T> = Pin<Box<dyn Future<Output = HarTexResult<T>> + Send + 'a>>;
