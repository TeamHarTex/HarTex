//! # `hartex_dbmani` - The HarTex PostgreSQL Database Manipulation Library
//!
//! The `hartex_dbmani` provides an interface for manipulating the PostgreSQL databases used by
//! HarTex Discord bot.

use std::{
    future::Future,
    pin::Pin
};

use hartex_core::error::HarTexResult;

pub mod whitelist;

/// # Typealias `PendingFuture`
/// 
/// Represents a pending future that is yet to return.
/// 
/// ## Generic Parameters
/// - `T`: represents the type that the pending future is to return.
type PendingFuture<T> = Pin<Box<dyn Future<Output = HarTexResult<T>>>>;
