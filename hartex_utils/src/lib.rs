//! # `hartex_utils` - The HarTex Utilities Library
//!
//! The `hartex_utils` library provides several utilities for the HarTex Discord bot.

#![feature(decl_macro)]

use std::{
    future::Future,
    pin::Pin
};

use hartex_core::error::HarTexResult;

pub const fn shard_id(guild_id: u64, shards: u64) -> u64 {
    (guild_id >> 22) % shards
}

/// # Typealias `FutureRetType`
///
/// A typealias for `Pin<Box<dyn Future<Output = HarTexResult<T>> + Send + 'a>>`.
pub type FutureRetType<'a, T> = Pin<Box<dyn Future<Output = HarTexResult<T>> + Send + 'a>>;
