//! # The `result` Module
//!
//! This module implements utilities for Result types.

use hartex_core::error::HarTexResult;

/// # Asynchronous Function `async_ok`
///
/// A utility function for a `Result<T, E>` wrapped in a `Future`.
#[allow(clippy::missing_errors_doc)] // this function never returns errors
#[allow(clippy::unused_async)]
pub async fn async_ok() -> HarTexResult<()> {
    Ok(())
}
