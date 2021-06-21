//! # The `level` Module
//!
//! This module contains definitions for various log levels used for the logger.

/// # Enum `LogLevel`
///
/// An enumeration represents various log levels used within the logger for the HarTex Discord bot.
pub enum LogLevel {
    /// # Enum Variant `LogLevel::Info`
    ///
    /// Represents the "general information" level. Generally used for displaying information that
    /// is not really related to debugging as such.
    Info,

    /// # Enum Variant `LogLevel::Debug`
    ///
    /// Represents the "debugging" level. Usually (almost all the time) used for debugging.
    Debug,

    /// # Enum Variant `LogLevel::Warn`
    ///
    /// Represents the "warning" level. Generally used for things that have gone wrong, but *not*
    /// so severe.
    Warn,

    /// # Enum Variant `LogLevel::Warn`
    ///
    /// Represents the "error" level. Specifically used when something goes really wrong, or when
    /// a fatal error occurs that the bot could not continue its current work and needs to be
    /// reported to the current user in the form of a Discord message.
    Error,

    /// # Enum Variant `LogLevel::Verbose`
    ///
    /// Represents the "verbose" level. This level is the most verbose, printing a *lot* of
    /// information. Can also be used for very useful debugging as if a bug occurs and its cause
    /// is difficult to track down.
    Verbose
}
