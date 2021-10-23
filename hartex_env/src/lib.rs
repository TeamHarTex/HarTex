//! # `hartex_env` - Environment Wrapper for HarTex Discord bot
//!
//! The `hartex_env` library various environments for different aspects of the HarTex Discord bot.

#![deny(clippy::pedantic, warnings, unsafe_code)]

use std::env;

use hartex_core::logging::tracing;

/// # Struct `DatabaseEnv`
///
/// Represents a collection of environment variables useful for the bot during database
/// manipulations.
pub struct DatabaseEnv {
    pub pgsql_credentials_guilds: Option<String>,
    pub pgsql_credentials_guildconfig: Option<String>
}

impl DatabaseEnv {
    /// # Static Method `DatabaseEnv::get`
    ///
    /// Retrieves the environment variables.
    pub fn get() -> Self {
        tracing::trace!("retrieving `PGSQL_CREDENTIALS_GUILDS` environment variable");
        let pgsql_credentials_guilds = env::var("PGSQL_CREDENTIALS_GUILDS").ok();

        tracing::trace!("retrieving `PGSQL_CREDENTIALS_GUILDCONFIG` environment variable");
        let pgsql_credentials_guildconfig = env::var("PGSQL_CREDENTIALS_GUILDCONFIG").ok();

        DatabaseEnv {
            pgsql_credentials_guilds,
            pgsql_credentials_guildconfig
        }
    }
}

/// # Struct `StartupEnv`
///
/// Represents a collection of environment variables useful for the bot during startup.
pub struct StartupEnv {
    pub application_id: Option<String>,
    pub bot_token: Option<String>
}

impl StartupEnv {
    /// # Static Method `StartupEnv::get`
    ///
    /// Retrieves the environment variables.
    pub fn get() -> Self {
        tracing::trace!("retrieving `APPLICATION_ID` environment variable");
        let application_id = env::var("APPLICATION_ID").ok();

        tracing::trace!("retrieving `BOT_TOKEN` environment variable");
        let bot_token = env::var("BOT_TOKEN").ok();

        StartupEnv {
            application_id,
            bot_token
        }
    }
}
