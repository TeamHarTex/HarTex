//! # `hartex_logging` - The HarTex Logging Library
//!
//! The `hartex_logging` library contains an implementation of a logger for the HarTex Discord bot.

use hartex_core::ansi::SgrParam;

pub mod level;

/// # Struct `Logger`
///
/// The main logger that HarTex Discord bot uses.
pub struct Logger;

impl Logger {
    /// # Static Method `Logger:log`
    ///
    /// Logs a message to the console with the provided log level (the `log_level` parameter).
    ///
    /// ## Generic Parameters
    /// - `T`; `Into<String>`
    ///
    /// ## Parameters
    ///
    /// - `message`, type `T`: the message to be loged
    /// - `log_level`, type `level::LogLevel`: the log level to use
    /// - `module`, type `Option<T>`: the module where the static method is invoked; can be `None`,
    ///                               and defaults to the `hartex_logging` module.
    pub fn log<T>(message: T, log_level: level::LogLevel, module: Option<T>)
    where
        T: Into<String> {
        match log_level {
            level::LogLevel::Info => Logger::info(message, module),
            _ => todo!()
        }
    }

    /// # Static Method `Logger::info`
    ///
    /// Logs a message to the console with the "information" log level.
    ///
    /// ## Generic Parameters
    /// - `T`; `Into<String>`
    ///
    /// ## Parameters
    /// - `message`, type `T`: the message to be logged
    /// - `module`, type `Option<T>`: the module the where the static method is invoked; can be
    ///                               `None`, and defaults to the `hartex_logging` module.
    pub fn info<T>(message: T, module: Option<T>)
    where
        T: Into<String> {
        let module_name = module.unwrap_or(module_path!());
    }
}
