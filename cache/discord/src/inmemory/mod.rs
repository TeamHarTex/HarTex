//! # `inmemory` Module
//!
//! This module provides an in-memory implementation of a Discord cache.
//!
//! This implementation is to be toggled with the `inmemory` feature of
//! this crate.

use hartex_cache_base::backend::Backend;

pub struct InMemoryBackend;

impl Backend for InMemoryBackend {}
