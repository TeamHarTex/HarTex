//! # `hartex_core` - The HarTex Core Library
//!
//! The `hartex_core` library contains the core functionality for the HarTex Discord bot.
//!
//! ## Features
//!
//! - `twilight-bundled`: bundles most of the `twilight` ecosystem of crates with the library,
//!                       removes the need to include the dependencies repeatedly across the
//!                       HarTex crates.

pub use ctrlc;

pub mod ansi;
#[cfg(feature = "twilight-bundled")]
pub mod discord;
pub mod error;
pub mod events;
pub mod time;

/// # Static `HARTEX_BUILD`
///
/// Represents the current version and build of HarTex Discord bot.
pub static HARTEX_BUILD: &'static str = "Version 1.19.0, 21H2 (Build 83)";
