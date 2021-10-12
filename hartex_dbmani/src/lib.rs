//! # `hartex_dbmani` - The HarTex PostgreSQL Database Manipulation Library
//!
//! The `hartex_dbmani` provides an interface for manipulating the PostgreSQL databases used by
//! HarTex Discord bot.

#![allow(non_snake_case)]
#![deny(clippy::pedantic)]
#![feature(format_args_capture)]

use std::{
    future::Future,
    pin::Pin
};

use hartex_core::error::HarTexResult;

pub mod guildconf;
pub mod whitelist;

/// # Typealias `PendingFuture`
///
/// Represents a pending future that is yet to return.
///
/// ## Generic Parameters
/// - `T`: represents the type that the pending future is to return.
type PendingFuture<T> = Pin<Box<dyn Future<Output = HarTexResult<T>>>>;
