//! # `hartex_core` - The `HarTex` Core Library
//!
//! The `hartex_core` library contains the core functionality for the `HarTex` Discord bot.
//!
//! ## Features
//!
//! - `twilight-bundled`: bundles most of the `twilight` ecosystem of crates with the library,
//!                       removes the need to include the dependencies repeatedly across the
//!                       `HarTex` crates.

#![deny(clippy::pedantic, warnings, unsafe_code)]
#![feature(format_args_capture)]

pub use ctrlc;

#[cfg(feature = "twilight-bundled")]
pub mod discord;
pub mod error;
pub mod events;
#[cfg(feature = "tracing-bundled")]
pub mod logging;
pub mod time;

/// # Static `HARTEX_BUILD`
///
/// Represents the current version and build of `HarTex` Discord bot.
pub static HARTEX_BUILD: &str = "Version 1.23.0-nightly-14-11-2021 (Build 518)";
