/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! Useful extensions for the `bitflags` crate.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use bitflags::Flags;

/// Extension trait for items implementing the `Flags` trait containing convenience methods
/// for various purposes.
pub trait FlagsExt: Flags {
    /// Obtain an instance of flags from their names.
    fn from_names(names: Vec<String>) -> Self {
        let mut flags = Self::empty();

        names
            .iter()
            .filter_map(|name| Self::from_name(name))
            .for_each(|flag| flags.insert(flag));

        flags
    }
}

impl<T> FlagsExt for T where T: Flags {}
