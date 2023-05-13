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

//! # Bors for HarTex
//!
//! A reimplementation of Bors in Rust for usage in the HarTex repository.

use hartex_log::log;
use tokio::runtime::Builder;

/// Entry point.
pub fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;
    hartex_log::initialize();

    actual_main()?;

    Ok(())
}

/// Actual entry point, building the runtime and other stuff.
fn actual_main() -> hartex_eyre::Result<()> {
    log::trace!("constructing runtime");
    let _ = Builder::new_multi_thread()
        .enable_all()
        .build()?;

    Ok(())
}
