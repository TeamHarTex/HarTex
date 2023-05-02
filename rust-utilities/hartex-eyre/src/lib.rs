/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Eyre Error Handling

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

mod constants;
mod handler;
mod hook;

pub use eyre;

/// Initialize the eyre error handlers.
#[allow(clippy::missing_errors_doc)]
pub fn initialize() -> Result<()> {
    hook::HookBuilder::new().install_hooks()
}

/// A convenience typealias for a Result with an eyre Report as its error type.
pub type Result<T, E = eyre::Report> = eyre::Result<T, E>;
