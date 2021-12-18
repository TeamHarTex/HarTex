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

//! # `hartex_locale` - Localization Facilities for `HarTex` Discord bot
//!
//! The `hartex_locale` crate contains translations of HarTex messages into various locales or
//! languages.

use std::{
    fs,
    path::Path
};

use hartex_core::error::HarTexResult;

/// # Struct `Locale`
///
/// A structure representing a locale.
pub struct Locale {
    file_buffer: String
}

impl Locale {
    /// # Static Method `Locale::load`
    ///
    /// Loads and constructs a locale structure from a language configuration file.
    pub fn load(path: &Path) -> HarTexResult<Self> {
        let mut file = fs::read_to_string(path)?;
        file = file
            .lines()
            .filter(|line| !line.starts_with(";") && !line.is_empty())
            .collect();

        Ok(Self {
            file_buffer: file
        })
    }

    /// # Instance Method `Locale::lang_id`
    ///
    /// Retrieves the language identifier from the current loaded language configuration file.
    pub fn lang_id(&self) -> String {
        let line = self
            .file_buffer
            .lines()
            .find(|line| line.starts_with("LanguageIdentifier: "))
            .unwrap();

        line.trim_start_matches("LanguageIdentifier: ").into_string()
    }
}
