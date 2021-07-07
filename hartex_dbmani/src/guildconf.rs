//! # The `guildconf` Module
//!
//! This module defines a database manipulation procedure to retrieve the TOML configuration of a
//! speciifc guild, and deserializing it into Rust structs so that it is usable in Rust code.

use std::{
    env,
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use sqlx::{
    postgres::PgPool,
    Row
};

use hartex_conftoml::TomlConfig;

use hartex_core::{
    discord::model::id::GuildId,
    error::{
        HarTexError,
        HarTexResult,
    }
};

use hartex_logging::Logger;

use crate::PendingFuture;

/// # Struct `GetGuildConfig`
///
/// Gets the guild configuration from the database.
pub struct GetGuildConfig {
    pending: Option<PendingFuture<TomlConfig>>,

    guild_id: GuildId
}

impl GetGuildConfig {
    /// # Constructor `GetGuildConfig::new`
    ///
    /// Creates a new `GetGuildConfig` with the provided `guild_id`.
    ///
    /// ## Parameters
    /// - `guild_id`, type `GuildId`: the guild id to get the configuration for.
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            pending: None,

            guild_id
        }
    }

    /// # Private Function `GetGuildConfig::start`
    ///
    /// Starts the future.
    fn start(&mut self) -> HarTexResult<()> {
        Logger::verbose(
            "executing future `GetGuildConfig`",
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        self.pending.replace(Box::pin(exec_future(self.guild_id)));

        Ok(())
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
async fn exec_future(guild_id: GuildId) -> HarTexResult<TomlConfig> {
    let db_credentials = match env::var("PGSQL_CREDENTIALS_GUILDCONFIG") {
        Ok(credentials) => credentials,
        Err(error) => {
            let message = format!("failed to get database credentials; error: {}", error);

            Logger::error(
                &message,
                Some(module_path!()),
                file!(),
                line!(),
                column!()
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
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            return Err(HarTexError::Custom {
                message
            });
        }
    };

    match sqlx::query(&format!("SELECT * FROM \"Guild{}\"; --", guild_id)).fetch_one(&connection)
        .await {
        Ok(row) => {
            let config = row.get::<String, &str>("TomlConfig");

            hartex_conftoml::from_string(config)
        },
        Err(error) => {
            let message = format!("failed to execute sql query; error `{:?}`", error);

            Logger::error(
                &message,
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            Err(HarTexError::Custom {
                message
            })
        }
    }
}
