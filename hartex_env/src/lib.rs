//! # `hartex_env` - Environment Wrapper for HarTex Discord bot
//!
//! The `hartex_env` library various environments for different aspects of the HarTex Discord bot.

use std::env;

use hartex_core::logging::tracing;

/// # Struct `StartupEnv`
///
/// Represents a collection of environment variables useful for the bot during startup.
pub struct StartupEnv {
    pub application_id: Option<String>,
    pub bot_token: Option<String>
}

impl StartupEnv {
    /// # Static Method `StartupEnv::get`
    pub fn get() -> Self {
        tracing::trace!("retrieving `APPLICATION_ID` environment variable");
        let application_id = env::var("APPLICATION_ID").ok();

        tracing::trace!("retrieving `BOT_TOKEN` environment variable");
        let bot_token = env::var("BOT_TOKEN").ok();

        StartupEnv {
            application_id,
            bot_token,
        }
    }
}
