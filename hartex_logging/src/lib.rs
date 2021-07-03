//! # `hartex_logging` - The HarTex Logging Library
//!
//! The `hartex_logging` library contains an implementation of a logger for the HarTex Discord bot.

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
    pub fn log(message: impl Into<String>, log_level: level::LogLevel, module: Option<&'static str>) {
        match log_level {
            level::LogLevel::Info => Logger::info(message, module),
            level::LogLevel::Debug => Logger::debug(message, module),
            level::LogLevel::Warn => Logger::warn(message, module),
            level::LogLevel::Error => Logger::error(message, module),
            level::LogLevel::Verbose => Logger::verbose(message, module)
        }
    }

    /// # Static Method `Logger::info`
    ///
    /// Logs a message to the console with the "information" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn info(message: impl Into<String>, module: Option<&'static str>) {
        let module_name = module.unwrap_or(module_path!());
        let mut params = SgrParam::BoldOrIncreasedIntensity.into_i32s();

        params.append(&mut SgrParam::SetColour {
            colour: AnsiColour::CustomU8 {
                n: 2
            },
            foreground: true
        }.into_i32s());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(params),
            level::LogLevel::Info.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name,
            message.into()
        );
    }

    /// # Static Method `Logger::debug`
    ///
    /// Logs a message to the console with the "debug" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str`: the module the where the static method is invoked; can be
    ///                                         `None`, and defaults to the `hartex_logging` module.
    pub fn debug(message: impl Into<String>, module: Option<&'static str>) {
        let module_name = module.unwrap_or(module_path!());
        let mut params = SgrParam::BoldOrIncreasedIntensity.into_i32s();

        params.append(&mut SgrParam::SetColour {
            colour: AnsiColour::CustomU8 {
                n: 33
            },
            foreground: true
        }.into_i32s());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(params),
            level::LogLevel::Debug.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name,
            message.into()
        );
    }

    /// # Static Method `Logger::warn`
    ///
    /// Logs a message to the console with the "warning" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn warn(message: impl Into<String>, module: Option<&'static str>) {
        let module_name = module.unwrap_or(module_path!());
        let mut params = SgrParam::BoldOrIncreasedIntensity.into_i32s();

        params.append(&mut SgrParam::SetColour {
            colour: AnsiColour::CustomU8 {
                n: 226
            },
            foreground: true
        }.into_i32s());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(params),
            level::LogLevel::Warn.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name,
            message.into()
        );
    }

    /// # Static Method `Logger::error`
    ///
    /// Logs a message to the console with the "error" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn error(message: impl Into<String>, module: Option<&'static str>) {
        let module_name = module.unwrap_or(module_path!());
        let mut params = SgrParam::BoldOrIncreasedIntensity.into_i32s();

        params.append(&mut SgrParam::SetColour {
            colour: AnsiColour::CustomU8 {
                n: 1
            },
            foreground: true
        }.into_i32s());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(params),
            level::LogLevel::Error.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name,
            message.into()
        );
    }

    /// # Static Method `Logger::verbose`
    ///
    /// Logs a message to the console with the "verbose" log level.
    ///
    /// ## Parameters
    /// - `message`, type `impl Into<String>`: the message to be logged
    /// - `module`, type `Option<&'static str>`: the module the where the static method is invoked; can be
    ///                                          `None`, and defaults to the `hartex_logging` module.
    pub fn verbose(message: impl Into<String>, module: Option<&'static str>) {
        let module_name = module.unwrap_or(module_path!());
        let mut params = SgrParam::BoldOrIncreasedIntensity.into_i32s();

        params.append(&mut SgrParam::SetColour {
            colour: AnsiColour::CustomU8 {
                n: 240
            },
            foreground: true
        }.into_i32s());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(params),
            level::LogLevel::Verbose.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name,
            message.into()
        );
    }
}
