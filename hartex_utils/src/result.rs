/* SPDX-License-Identifier: GPL-2.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # The `result` Module
//!
//! This module implements utilities for Result types.

use hartex_core::error::HarTexResult;

/// # Asynchronous Function `async_ok`
///
/// A utility function for a `Result<T, E>` wrapped in a `Future`.
#[allow(clippy::missing_errors_doc)] // this function never returns errors
#[allow(clippy::unused_async)]
pub async fn async_ok() -> HarTexResult<()> {
    Ok(())
}
