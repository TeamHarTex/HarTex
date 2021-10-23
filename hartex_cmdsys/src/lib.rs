//! # `hartex_cmdsys` - The HarTex Command System Library
//!
//! The `hartex_cmdsys` library contains an implementation of a command system for HarTex Discord
//! bot, including a command parser, as well as various utilities for the implementation.

#![deny(clippy::pedantic, warnings, unsafe_code)]

pub mod command;
pub mod context;
pub mod framework;
