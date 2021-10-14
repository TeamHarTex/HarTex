//! # `hartex_dbmani` - The HarTex PostgreSQL Database Manipulation Library
//!
//! The `hartex_dbmani` provides an interface for manipulating the PostgreSQL databases used by
//! HarTex Discord bot.

#![allow(non_snake_case)]
#![deny(clippy::pedantic, warnings)]
#![feature(format_args_capture)]
#![feature(once_cell)]

use std::{
    future::Future,
    lazy::SyncLazy,
    pin::Pin
};

use hartex_core::error::HarTexResult;
use hartex_env::DatabaseEnv;

pub mod guildconf;
pub mod whitelist;

/// # Static `DATABASE_ENV`
///
/// Useful environment variables for database manipulation
static DATABASE_ENV: SyncLazy<DatabaseEnv> = SyncLazy::new(|| DatabaseEnv::get());

/// # Typealias `PendingFuture`
///
/// Represents a pending future that is yet to return.
///
/// ## Generic Parameters
/// - `T`: represents the type that the pending future is to return.
type PendingFuture<T> = Pin<Box<dyn Future<Output = HarTexResult<T>> + Send>>;

/// # Function `init_env`
///
/// Initializes the environment variables for later use, must be called in the "entry point" in
/// the `hartex_driver` crate for the environment variables to be usable.
pub fn init_env() {
    SyncLazy::force(&DATABASE_ENV);
}
