//! # `hartex` - The HarTex Binary Crate
//!
//! This crate contains the main function which calls the `hartex_main` "main function" in the
//! `hartex_driver` crate which glues everything together.

use std::time::Duration;

use tokio::runtime::Builder;

use hartex_core::error::HarTexResult;

pub fn main() -> HarTexResult<'static, ()> {
    let tokio_runtime = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .thread_name("hartex")
        .build()
        .unwrap();

    tokio_runtime.block_on(async move {
        hartex_driver::hartex_main().await
    })?;
    tokio_runtime.shutdown_timeout(Duration::from_millis(100));

    Ok(())
}
