//! # The `guildconf` Module
//!
//! This module defines a database manipulation procedure to retrieve the TOML configuration of a
//! specific guild, and deserializing it into Rust structs so that it is usable in Rust code.

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
    },
    logging::tracing
};

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
        let span = tracing::trace_span!("database manipulation: get guild config");
        span.in_scope(|| {
            tracing::trace!("executing future `GetGuildConfig`");
        });


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
    let span = tracing::trace_span!(parent: None, "database manipulation: get guild config");

    let db_credentials = match env::var("PGSQL_CREDENTIALS_GUILDCONFIG") {
        Ok(credentials) => credentials,
        Err(error) => {
            let message = format!("failed to get database credentials; error: {error}");

            span.in_scope(|| {
                tracing::error!("{message}", message = &message)
            });

            return Err(HarTexError::Custom {
                message
            });
        }
    };

    span.in_scope(|| {
        tracing::trace!("connecting to database...");
    });

    let connection = match PgPool::connect(&db_credentials).await {
        Ok(pool) => pool,
        Err(error) => {
            let message = format!("failed to connect to postgres database pool; error: `{error:?}`");

            span.in_scope(|| {
                tracing::error!("{message}", message = &message)
            });

            return Err(HarTexError::Custom {
                message
            });
        }
    };

    span.in_scope(|| {
        tracing::trace!("executing sql query...");
    });

    match sqlx::query(&format!("SELECT * FROM \"Guild{guild_id}\"; --")).fetch_one(&connection)
        .await {
        Ok(row) => {
            let config = row.get::<String, &str>("TomlConfig");

            let decoded = match base64::decode(config) {
                Ok(bytes) => bytes,
                Err(error) => {
                    let message = format!("failed to decode base64; error: `{error:?}`");

                    span.in_scope(|| {
                        tracing::error!("{message}", message = &message);
                    });

                    return Err(HarTexError::Base64DecodeError {
                        error
                    });
                }
            };

            span.in_scope(|| {
                tracing::trace!("deserializing toml config...");
            });

            hartex_conftoml::from_string(match String::from_utf8(decoded) {
                Ok(string) => string,
                Err(error) => {
                    let message = format!("failed to construct utf-8 string; error: `{error:?}`");

                    span.in_scope(|| {
                        tracing::error!("{message}", message = &message);
                    });

                    return Err(HarTexError::Utf8ValidationError {
                        error
                    });
                }
            })
        },
        Err(error) => {
            let message = format!("failed to execute sql query; error `{error:?}`");

            span.in_scope(|| {
                tracing::error!("{message}", message = &message);
            });

            Err(HarTexError::Custom {
                message
            })
        }
    }
}
