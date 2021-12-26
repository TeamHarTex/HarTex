/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `result` Module
//!
//! This module implements utilities for Result types.

use hartex_base::error::HarTexResult;

/// # Asynchronous Function `async_ok`
///
/// A utility function for a `Result<T, E>` wrapped in a `Future`.
#[allow(clippy::missing_errors_doc)] // this function never returns errors
#[allow(clippy::unused_async)]
pub async fn async_ok() -> HarTexResult<()> {
    Ok(())
}
