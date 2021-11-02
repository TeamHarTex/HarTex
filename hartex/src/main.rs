//! # `hartex` - The `HarTex` Binary Crate
//!
//! This crate contains the main function which calls the `hartex_main` "main function" in the
//! `hartex_driver` crate which glues everything together.

#![deny(clippy::pedantic, warnings, unsafe_code)]

use std::time::Duration;

use hartex_core::{
    error::HarTexResult,
    logging::{
        tracing,
        tracing_subscriber::{
            self,
            fmt::time::UtcTime,
            EnvFilter
        }
    }
};
use tokio::runtime::Builder;

pub fn main() -> HarTexResult<()> {
    // loads the .env file to obtain environment variables
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(UtcTime::rfc_3339())
        .init();

    let tokio_runtime = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .thread_name("hartex")
        .build()
        .unwrap();

    tokio_runtime.block_on(async move {
        tracing::trace!("executing hartex_driver entrypoint");

        hartex_driver::hartex_main().await
    })?;
    tokio_runtime.shutdown_timeout(Duration::from_millis(100));

    Ok(())
}
