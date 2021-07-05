//! # `hartex_utils` - The HarTex Utilities Library
//!
//! The `hartex_utils` library provides several utilities for the HarTex Discord bot.

use std::{
    future::Future,
    pin::Pin
};

use hartex_core::error::HarTexResult;

/// # Typealias `FutureRetType`
///
/// A typealias for `Pin<Box<dyn Future<Output = HarTexResult<T>> + Send + 'a>>`.
pub type FutureRetType<'a, T> = Pin<Box<dyn Future<Output = HarTexResult<T>> + Send + 'a>>;
