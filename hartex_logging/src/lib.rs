//! # `hartex_logging` - The HarTex Logging Library
//!
//! The `hartex_logging` library contains an implementation of a logger for the HarTex Discord bot.

#![feature(format_args_capture)]

use hartex_core::{
    ansi::{
        ansi_display,
        AnsiColour,
        SgrParam
    },
    time::Local
};

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
    /// ## Parameters
    ///
    /// - `message`, type `impl Into<String>`: the message to be loged
    /// - `log_level`, type `level::LogLevel`: the log level to use
    /// - `module`, type `Option<&'static str>`: the module where the static method is invoked; can be `None`,
    ///                                          and defaults to the `hartex_logging` module.
    pub fn log(message: impl Into<String>, log_level: level::LogLevel, module: Option<&'static str>, file: &'static str, line: u32, column: u32) {
        let module_name = module.unwrap_or(module_path!());
        let mut params = SgrParam::BoldOrIncreasedIntensity.into_i32s();

        params.append(&mut SgrParam::SetColour {
            colour: AnsiColour::CustomU8 {
                n: match log_level {
                    level::LogLevel::Info => 2,
                    level::LogLevel::Debug => 33,
                    level::LogLevel::Warn => 226,
                    level::LogLevel::Error => 1,
                    level::LogLevel::Verbose => 240
                }
            },
            foreground: true
        }.into_i32s());

        println!(
            "[HarTex v{version}: {now}+08:00] [{ansi}{level}{reset}] [{module_name}] [{file}:{line}:{column}] {message}",
            version = env!("CARGO_PKG_VERSION"),
            now = Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi = ansi_display(params),
            level = log_level.display(),
            reset = ansi_display(SgrParam::Reset.into_i32s()),
            file = file.strip_prefix(r"D:\Projects\HarTexBot\HarTex-rust-discord-bot\").unwrap().replace(r"\", "/"),
            message = message.into()
        );
    }

    /// # Static Method `Logger::info`
    ///
    /// Logs a message to the console with the "information" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn info(message: impl Into<String>, module: Option<&'static str>, file: &'static str, line: u32, column: u32) {
        Self::log(message, level::LogLevel::Info, module, file, line, column)
    }

    /// # Static Method `Logger::debug`
    ///
    /// Logs a message to the console with the "debug" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str`: the module the where the static method is invoked; can be
    ///                                         `None`, and defaults to the `hartex_logging` module.
    pub fn debug(message: impl Into<String>, module: Option<&'static str>, file: &'static str, line: u32, column: u32) {
        Self::log(message, level::LogLevel::Debug, module, file, line, column)
    }

    /// # Static Method `Logger::warn`
    ///
    /// Logs a message to the console with the "warning" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn warn(message: impl Into<String>, module: Option<&'static str>, file: &'static str, line: u32, column: u32) {
        Self::log(message, level::LogLevel::Warn, module, file, line, column)
    }

    /// # Static Method `Logger::error`
    ///
    /// Logs a message to the console with the "error" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn error(message: impl Into<String>, module: Option<&'static str>, file: &'static str, line: u32, column: u32) {
        Self::log(message, level::LogLevel::Error, module, file, line, column)
    }

    /// # Static Method `Logger::verbose`
    ///
    /// Logs a message to the console with the "verbose" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn verbose(message: impl Into<String>, module: Option<&'static str>, file: &'static str, line: u32, column: u32) {
        Self::log(message, level::LogLevel::Verbose, module, file, line, column)
    }
}
