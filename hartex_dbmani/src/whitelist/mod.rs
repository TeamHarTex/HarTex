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
    postgres::PgPool,
    Postgres
};

use hartex_core::error::{
    HarTexError,
    HarTexResult
};

use hartex_logging::Logger;

use crate::{
    whitelist::model::WhitelistedGuild,
    PendingFuture
};

pub mod model;

/// # Struct `GetWhitelistedGuilds`
///
/// Gets the whitelisted guilds of the bot.
pub struct GetWhitelistedGuilds {
    pending: Option<PendingFuture<DashMap<String, u64>>>
}

impl GetWhitelistedGuilds {
    /// # Private Function `GetWhitelistedGuilds::start`
    ///
    /// Starts the future.
    fn start(&mut self) -> HarTexResult<()> {
        Logger::verbose(
            "executing future `GetWhitelistedGuilds`",
            Some(module_path!())
        );

        self.pending.replace(Box::pin(exec_future()));

        Ok(())
    }
}

impl Default for GetWhitelistedGuilds {
    fn default() -> Self {
        Self {
            pending: None
        }
    }
}

impl Future for GetWhitelistedGuilds {
    type Output = HarTexResult<DashMap<String, u64>>;

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


/// # Asynchronous Function `exec_future`
///
/// Executes the future.
async fn exec_future() -> HarTexResult<DashMap<String, u64>> {
    let db_credentials = match env::var("PGSQL_CREDENTIALS_GUILDS") {
        Ok(credentials) => credentials,
        Err(error) => {
            let message = format!("failed to get database credentials; error: {}", error);

            Logger::error(
                &message,
                Some(module_path!())
            );

            return Err(HarTexError::Custom {
                message
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
                message
            });
        }
    };

    match sqlx::query_as::<Postgres, WhitelistedGuild>(r#"SELECT * FROM public."Whitelist""#).fetch_all(&connection).await {
        Ok(guilds) => {
            let mut map = DashMap::new();

            guilds.iter().for_each(|guild| {
                map.insert(guild.GuildName.to_string(), guild.GuildId);
            });

            Ok(map)
        }
        Err(error) => {
            let message = format!("failed to execute sql query; error `{:?}`", error);

            Logger::error(
                &message,
                Some(module_path!())
            );

            Err(HarTexError::Custom {
                message
            })
        }
    }
}
