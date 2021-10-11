//! # The `env_setup` Module
//!
//! This module provides various constructs for the environment setup of the bot.

use std::{
    env,
    process
};

use hartex_core::logging::tracing;

/// # Struct `Environment`
///
/// Represents the environment variables useful for the bot.
pub struct Environment {
    pub application_id: String,
    pub token: String
}

/// # Function `environment_setup`
///
/// Returns an instance of `Environment` containing the environment variables (if there are no
/// failures during the retrieval of such variables).
pub fn environment_setup() -> Environment {
    tracing::trace!("environment variables are loaded");

    let token = match env::var("HARTEX_TOKEN") {
        Ok(token) => token,
        Err(var_error) => {
            tracing::error!("could not retrieve bot token from environment variables: {var_error}");

            process::exit(-1)
        }
    };

    tracing::trace!("successfully retrieved bot token");

    let application_id = match env::var("APPLICATION_ID") {
        Ok(id) => id,
        Err(var_error) => {
            tracing::error!(
                "could not retrieve application id from environment variables: {var_error}"
            );

            process::exit(-1)
        }
    };

    tracing::trace!("successfully retrieved application id");

    Environment {
        application_id,
        token
    }
}
