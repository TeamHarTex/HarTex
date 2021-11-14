//! # `hartex_env` - Environment Wrapper for `HarTex` Discord bot
//!
//! The `hartex_env` library various environments for different aspects of the `HarTex` Discord bot.

#![deny(clippy::pedantic, warnings, unsafe_code)]

use std::{
    env,
    num::NonZeroU64
};

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
    #[must_use]
    pub fn get() -> Self {
        tracing::trace!("retrieving `PGSQL_CREDENTIALS_GUILDS` environment variable");
        let pgsql_credentials_guilds = env::var("PGSQL_CREDENTIALS_GUILDS").ok();

        tracing::trace!("retrieving `PGSQL_CREDENTIALS_GUILDCONFIG` environment variable");
        let pgsql_credentials_guildconfig = env::var("PGSQL_CREDENTIALS_GUILDCONFIG").ok();

        Self {
            pgsql_credentials_guilds,
            pgsql_credentials_guildconfig
        }
    }
}

/// # Struct `PluginEnv`
///
/// Represents a collection of environment variables useful for the bot in plugins.
pub struct PluginEnv {
    pub global_administrator_uid: Option<NonZeroU64>
}

impl PluginEnv {
    #[must_use]
    pub fn get() -> Self {
        tracing::trace!("retrieving `GLOBAL_ADMINISTRATOR_UID` environment variable");
        let global_administrator_uid = env::var("GLOBAL_ADMINISTRATOR_UID").ok();

        Self {
            global_administrator_uid: global_administrator_uid
                .map(|uid| NonZeroU64::new(uid.parse().unwrap()).unwrap())
        }
    }
}

/// # Struct `StartupEnv`
///
/// Represents a collection of environment variables useful for the bot during startup.
pub struct StartupEnv {
    pub application_id: Option<NonZeroU64>,
    pub bot_token: Option<String>
}

impl StartupEnv {
    /// # Static Method `StartupEnv::get`
    ///
    /// Retrieves the environment variables.
    #[must_use]
    pub fn get() -> Self {
        tracing::trace!("retrieving `APPLICATION_ID` environment variable");
        let application_id = env::var("APPLICATION_ID").ok();

        tracing::trace!("retrieving `BOT_TOKEN` environment variable");
        let bot_token = env::var("BOT_TOKEN").ok();

        Self {
            application_id: application_id.map(|id| NonZeroU64::new(id.parse().unwrap()).unwrap()),
            bot_token
        }
    }
}
