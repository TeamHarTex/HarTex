//! # `hartex_utils` - The `HarTex` Utilities Library
//!
//! The `hartex_utils` library provides several utilities for the `HarTex` Discord bot.

#![deny(clippy::pedantic, warnings, unsafe_code)]
#![feature(format_args_capture)]

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
