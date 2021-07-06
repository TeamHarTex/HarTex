//! # The `guildconf` Module
//!
//! This module defines a database manipulation procedure to retrieve the TOML configuration of a
//! speciifc guild, and deserializing it into Rust structs so that it is usable in Rust code.

use std::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use hartex_conftoml::TomlConfig;

use hartex_core::error::HarTexResult;

use hartex_logging::Logger;

use crate::PendingFuture;

/// # Struct `GetGuildConfig`
/// 
/// Gets the guild configuration from the database.
pub struct GetGuildConfig {
    pending: Option<PendingFuture<TomlConfig>>
}

impl GetGuildConfig {
    /// # Private Function `GetGuildConfig::start`
    ///
    /// Starts the future.
    fn start(&mut self) -> HarTexResult<()> {
        Logger::verbose(
            "executing future `GetGuildConfig`",
            Some(module_path!())
        );

        self.pending.replace(Box::pin(exec_future()));

        Ok(())
    }
}

impl Default for GetGuildConfig {
    fn default() -> Self {
        Self {
            pending: None
        }
    }
}

impl Future for GetGuildConfig {
    type Output = HarTexResult<TomlConfig>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(pending) = self.pending.as_mut() {
                return pending.as_mut().poll(cx);
            }

            if let Err(error) = self.start() {
                return Poll::Ready(Err(error))
            }
        }
    }
}

unsafe impl Send for GetGuildConfig { }

/// # Asynchronous Function `exec_future`
///
/// Executes the future.
async fn exec_future() -> HarTexResult<TomlConfig> {
    todo!()
}
