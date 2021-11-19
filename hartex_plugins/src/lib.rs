//! # `hartex_plugins` - The `HarTex` Plugins Library
//!
//! The `hartex_plugins` library implements the bot plugins.

#![deny(clippy::pedantic, warnings, unsafe_code)]
#![feature(once_cell)]

use std::lazy::SyncLazy;

use hartex_env::PluginEnv;

pub mod globadmin_only;
pub mod global;
pub mod information;
pub mod utilities;

/// # Static `PLUGIN_ENV`
///
/// Useful environment variables for various bot plugins
pub static PLUGIN_ENV: SyncLazy<PluginEnv> = SyncLazy::new(PluginEnv::get);

/// # Function `init_env`
///
/// Initializes the environment variables for later use, must be called in the "entry point" in
/// the `hartex_driver` crate for the environment variables to be usable.
pub fn init_env() {
    SyncLazy::force(&PLUGIN_ENV);
}
