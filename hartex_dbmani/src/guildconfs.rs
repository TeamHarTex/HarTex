//! # The `guildconfs` Module
//!
//! This module defines a database manipulation procedure to retrieve the TOML configurations of
//! all guids, and deserializing them into Rust structs so that it is usable in Rust code.

use std::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use hartex_conftoml::TomlConfig;
use hartex_core::{
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing
};
use sqlx::{
    PgPool,
    Row
};

use crate::{
    PendingFuture,
    DATABASE_ENV
};

/// # Struct `GetGuildConfig`
///
/// Gets the all guild configurations from the database.
pub struct GetGuildConfigs {
    pending: Option<PendingFuture<Vec<TomlConfig>>>
}

impl GetGuildConfigs {
    /// # Private Function `GetGuildConfigs::start`
    ///
    /// Starts the future.
    #[allow(clippy::unnecessary_wraps)]
    fn start(&mut self) -> HarTexResult<()> {
        let span = tracing::trace_span!(parent: None, "database manipulation: get guild configs");
        span.in_scope(|| tracing::trace!("executing future `GetGuildConfigs`"));

        self.pending.replace(Box::pin(exec_future()));

        Ok(())
    }
}

impl Future for GetGuildConfigs {
    type Output = HarTexResult<Vec<TomlConfig>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(pending) = self.pending.as_mut() {
                return pending.as_mut().poll(cx);
            }

            if let Err(error) = self.start() {
                return Poll::Ready(Err(error));
            }
        }
    }
}

/// # Asynchronous Function `exec_future`
///
/// Executes the future.
async fn exec_future() -> HarTexResult<Vec<TomlConfig>> {
    let span = tracing::trace_span!(parent: None, "database manipulation: get guild configs");

    let db_credentials = match &DATABASE_ENV.pgsql_credentials_guildconfig {
        Some(credentials) => credentials,
        None => {
            span.in_scope(|| {
                tracing::error!(
                    "the environment variable `PGSQL_CREDENTIALS_GUILDCONFIG` is not set"
                );
            });

            return Err(HarTexError::Custom {
                message: String::from(
                    "the environment variable `PGSQL_CREDENTIALS_GUILDCONFIG` is not set"
                )
            });
        }
    };

    span.in_scope(|| tracing::trace!("connecting to database..."));

    let connection = match PgPool::connect(db_credentials).await {
        Ok(pool) => pool,
        Err(error) => {
            let message =
                format!("failed to connect to postgres database pool; error: `{error:?}`");

            span.in_scope(|| tracing::error!("{message}", message = &message));

            return Err(HarTexError::Custom {
                message
            });
        }
    };

    span.in_scope(|| tracing::trace!("executing sql query..."));

    match sqlx::query(
        r"
CREATE OR REPLACE FUNCTION PUBLIC.IterTables()
    RETURNS SETOF RECORD
    LANGUAGE 'plpgsql'
    AS $FN$
DECLARE
    PgsqlTable RECORD;
BEGIN
    FOR PgsqlTable IN SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES
            WHERE TABLE_SCHEMA = 'public'
    LOOP
        RETURN QUERY EXECUTE 'SELECT * FROM ' || QUOTE_IDENT(PgsqlTable.TABLE_NAME);
    END LOOP;
END $FN$;

SELECT * FROM IterTables() f(TomlConfig TEXT)"
    )
    .fetch_all(&connection)
    .await
    {
        Ok(rows) => {
            let mut configs = vec![];

            for row in rows {
                let config = row.get::<String, &str>("TomlConfig");

                let decoded = match base64::decode(config) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        let message = format!("failed to decode base64; error: `{error:?}`");

                        span.in_scope(|| tracing::error!("{message}", message = &message));

                        return Err(HarTexError::Base64DecodeError {
                            error
                        });
                    }
                };

                span.in_scope(|| tracing::trace!("deserializing toml config..."));

                configs.push(hartex_conftoml::from_str(
                    match std::str::from_utf8(&*decoded) {
                        Ok(string) => string,
                        Err(error) => {
                            let message =
                                format!("failed to construct utf-8 string; error: `{error:?}`");

                            span.in_scope(|| tracing::error!("{message}", message = &message));

                            return Err(HarTexError::Utf8ValidationError {
                                error
                            });
                        }
                    }
                )?);
            }

            Ok(configs)
        }
        Err(error) => {
            let message = format!("failed to execute sql query; error `{error:?}`");

            span.in_scope(|| tracing::error!("{message}", message = &message));

            Err(HarTexError::Custom {
                message
            })
        }
    }
}
