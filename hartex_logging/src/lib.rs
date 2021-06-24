//! # `hartex_logging` - The HarTex Logging Library
//!
//! The `hartex_logging` library contains an implementation of a logger for the HarTex Discord bot.

use hartex_core::{
    ansi::{
        ansi_display,
        AnsiColour,
        SgrParam
    },
    time::Local,
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

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(SgrParam::SetColour {
                colour: AnsiColour::CustomU8 {
                    n: 2
                },
                foreground: true
            }.into_i32s()),
            level::LogLevel::Info.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name.into(),
            message.into()
        );
    }

    /// # Static Method `Logger::debug`
    ///
    /// Logs a message to the console with the "debug" log level.
    ///
    /// ## Generic Parameters
    /// - `T`; `Into<String>`
    ///
    /// ## Parameters
    /// - `message`, type `T`: the message to be logged
    /// - `module`, type `Option<T>`: the module the where the static method is invoked; can be
    ///                               `None`, and defaults to the `hartex_logging` module.
    pub fn debug<T>(message: T, module: Option<T>)
    where
        T: Into<String> {
        let module_name = module.unwrap_or(module_path!());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(SgrParam::SetColour {
                colour: AnsiColour::CustomU8 {
                    n: 33
                },
                foreground: true
            }.into_i32s()),
            level::LogLevel::Debug.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name.into(),
            message.into()
        );
    }

    /// # Static Method `Logger::warn`
    ///
    /// Logs a message to the console with the "warning" log level.
    ///
    /// ## Generic Parameters
    /// - `T`; `Into<String>`
    ///
    /// ## Parameters
    /// - `message`, type `T`: the message to be logged
    /// - `module`, type `Option<T>`: the module the where the static method is invoked; can be
    ///                               `None`, and defaults to the `hartex_logging` module.
    pub fn warn<T>(message: T, module: Option<T>)
    where
        T: Into<String> {
        let module_name = module.unwrap_or(module_path!());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(SgrParam::SetColour {
                colour: AnsiColour::CustomU8 {
                    n: 226
                },
                foreground: true
            }.into_i32s()),
            level::LogLevel::Warn.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name.into(),
            message.into()
        );
    }

    /// # Static Method `Logger::error`
    ///
    /// Logs a message to the console with the "error" log level.
    ///
    /// ## Generic Parameters
    /// - `T`; `Into<String>`
    ///
    /// ## Parameters
    /// - `message`, type `T`: the message to be logged
    /// - `module`, type `Option<T>`: the module the where the static method is invoked; can be
    ///                               `None`, and defaults to the `hartex_logging` module.
    pub fn error<T>(message: T, module: Option<T>)
    where
        T: Into<String> {
        let module_name = module.unwrap_or(module_path!());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(SgrParam::SetColour {
                colour: AnsiColour::CustomU8 {
                    n: 1
                },
                foreground: true
            }.into_i32s()),
            level::LogLevel::Error.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name.into(),
            message.into()
        );
    }

    /// # Static Method `Logger::verbose`
    ///
    /// Logs a message to the console with the "verbose" log level.
    ///
    /// ## Generic Parameters
    /// - `T`; `Into<String>`
    ///
    /// ## Parameters
    /// - `message`, type `T`: the message to be logged
    /// - `module`, type `Option<T>`: the module the where the static method is invoked; can be
    ///                               `None`, and defaults to the `hartex_logging` module.
    pub fn verbose<T>(message: T, module: Option<T>)
    where
        T: Into<String> {
        let module_name = module.unwrap_or(module_path!());

        println!(
            "[HarTex v{}: {}+08:00] [{}{}{}] [{}] {}",
            env!("CARGO_PKG_VERSION"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ansi_display(SgrParam::SetColour {
                colour: AnsiColour::CustomU8 {
                    n: 1
                },
                foreground: true
            }.into_i32s()),
            level::LogLevel::Verbose.display(),
            ansi_display(SgrParam::Reset.into_i32s()),
            module_name.into(),
            message.into()
        );
    }
}
