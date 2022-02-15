/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The HarTex Base Library
//!
//! The HarTex Base Library is the sole foundation of the broad codebase of HarTex, containing a
//! set of minimal abstractions for the HarTex ecosystem of libraries (and binaries). Core types
//! like [`Error`] type for error definitions, [`Result<T>`] type for error handling, and many
//! other things, which will be individually listed in the following section.
//!
//! ## What is in the Base Library?
//!
//! ### Re-exports of the Twilight Ecosystem
//!
//! HarTex is built upon a Rust Discord API wrapper library called "Twilight". For more information,
//! you may visit:
//!
//!  - Twilight's Official Website: <https://twilight.rs/>
//!  - Twilight's Official GitHub Repository: <https://github.com/twilight-rs/twilight>
//!  - Twilight's API Documentation: <https://api.twilight.rs/twilight/index.html>
//!
//! Due to the design nature of Twilight (split into an ecosystem of crates, where the individual
//! crates serve different functionality), it is deemed useful, or mandatory, to provide global
//! re-exports to the libraries in the [`discord`] module to reduce clumsiness over specifying
//! commonly used crates from the Twilight Ecosystem many times throughout the codebase of HarTex.
//!
//! This component of the Base Library is gated behind multiple feature flags (specifically one per
//! re-exported crate from the Twilight Ecosystem), for example `discord-gateway` and
//! `discord-model` to enable the re-exports of `twilight-gateway` and `twilight-model` from the
//! Twilight Ecosystem respectively. This is to avoid re-exporting the entirety of the Twilight
//! Ecosystem when it is not required at all and reduces clumsiness.
//!
//! ### Error Handling Facilities
//!
//! The HarTex Base Library provides an [`Error`] type as well as a [`Result<T>`] type for error
//! handling throughout the codebase in the [`error`] module. [`Result<T>`] in particular is a
//! typealias over [`Result<T, E>` from The Rust Standard Library], with the `E` generic parameter
//! set to the aforementioned [`Error`] type which is guaranteed to be used almost throughout the
//! codebase.
//!
//! There are [`From`] implementations for various other error types to be handed in the HarTex
//! codebase for usage of the `?` operator when dealing with [`Result<T>`]s, which would be useful
//! for returning an [`Err`] in functions instantly if some operation in that function returns an
//! [`Err`]. It is to be noted that the `?` operator is only usable on [`Result<T>`]s when the
//! function to use the operator in itself returns a [`Result<T>`].
//!
//! ### Logging Facilities
//!
//! The HarTex Base Library contains a [`logging`] module, and has an [`init()`] function for
//! invocation in binaries in the codebase to initialize logging to the standard output stream (and
//! possibly to a file, if also specified in the logging configuration which can be modified).
//!
//! The configured logging implementation used by HarTex is [`log4rs`].
//!
//! This component of the Base Library is also gated behind a feature flag - `logging`. It is to
//! avoid including the required dependencies for logging when the crate (that includes the Base
//! Library) does not need any logging implementation at all.
//!
//! ## Enabling Optional Feature Flags
//!
//! To enable these feature flags, you may follow the following instructions to do so:
//!
//! ### Pulling The HarTex Base Library from Git
//!
//! If you are pulling the Base Library from Git, you may use the following line in your
//! `Cargo.toml` file:
//! ```toml,no_run
//! base = { git = "https://github.com/HarTexTeam/HarTex-rust-discord-bot.git", branch = "stable", features = [<features to enable>] }
//! ```
//!
//! That will pull the code of the Base Library from the `stable` branch of the Git repository. If
//! you want to use latest (possible breaking) changes, you may modify the branch to pull from:
//!  `Cargo.toml` file:
//! ```toml,no_run
//! base = { git = "https://github.com/HarTexTeam/HarTex-rust-discord-bot.git", branch = "nightly", features = [<features to enable>] }
//! ```
//!
//! This will pull the code from the `nightly` branch, where latest changes are merged into the
//! source tree.
//!
//! ### Using a Local Copy/Checkout of the Base Library
//!
//! If you are using a local copy/checkout of the Base Library, you can just specify the path to
//! the Base Library on your local system (you may use absolute or relative file paths) and specify
//! the needed features:
//! ```toml,no_run
//! base = { path = "<path to the base library>", features = [<features to enable>] }
//! ```
//!
//! [`discord`]: crate::discord
//! [`error`]: crate::error
//! [`Error`]: crate::error::Error
//! [`init()`]: crate::logging::init
//! [`log4rs`]: https://github.com/estk/log4rs
//! [`Result<T>`]: crate::error::Result
//! [`Result<T, E>` from The Rust Standard Library]: https://doc.rust-lang.org/std/result/enum.Result.html

#![deny(clippy::pedantic)]
#![deny(warnings)]

pub mod discord;
pub mod error;
#[cfg(feature = "logging")]
pub mod logging;
