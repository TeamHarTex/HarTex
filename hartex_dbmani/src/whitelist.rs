//! # The `whitelist` Module
//!
//! This module defines a database manipulation procedure for obtaining the whitelisted guilds of
//! the bot for checking whitelists.

use std::{
    env,
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use dashmap::DashMap;

use sqlx::{
    error::Result as SqlxResult,
    postgres::{
        PgPool,
        PgRow
    },
    Row
};

use hartex_core::error::{
    HarTexError,
    HarTexResult
};

use hartex_logging::Logger;

use crate::PendingFuture;

/// # Struct `GetWhitelistedGuilds`
///
/// Gets the whitelisted guilds of the bot.
pub struct GetWhitelistedGuilds<'a> {
    pending: Option<PendingFuture<'a, DashMap<&'a str, u64>>>
}

impl<'a> Default for GetWhitelistedGuilds<'a> {
    fn default() -> Self {
        Self {
            pending: None
        }
    }
}

impl<'a> Future for GetWhitelistedGuilds<'a> {
    type Output = HarTexResult<'a, DashMap<&'a str, u64>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}


/// # Asynchronous Function `exec_future`
///
/// Executes the future.
async fn exec_future<'a>() -> HarTexResult<'a, DashMap<&'a str, u64>> {
    let db_credentials = match env::var("PGSQL_CREDENTIALS_GUILDS") {
        Ok(credentials) => credentials,
        Err(error) => {
            let message = format!("failed to get database credentials; error: {}", error);

            Logger::error(
                &message,
                Some(module_path!())
            );

            return Err(HarTexError::Custom {
                message: &message
            });
        }
    };

    let connection = match PgPool::connect(&db_credentials).await {
        Ok(pool) => pool,
        Err(error) => {
            let message = format!("failed to connect to postgres database pool; error: `{:?}`", error);

            Logger::error(
                &message,
                Some(module_path!())
            );

            return Err(HarTexError::Custom {
                message: &message
            });
        }
    };
}
