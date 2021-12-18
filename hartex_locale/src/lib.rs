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
    collections::HashMap,
    fs,
    path::Path
};

use hartex_core::error::{
    HarTexError,
    HarTexResult
};

/// # Struct `Locale`
///
/// A structure representing a locale.
#[derive(Clone)]
pub struct Locale {
    file_map: HashMap<String, String>
}

impl Locale {
    /// # Static Method `Locale::load`
    ///
    /// Loads and constructs a locale structure from a language configuration file.
    pub fn load(path: &Path) -> HarTexResult<Self> {
        let mut file = fs::read_to_string(path)?;
        let mut before_validation = file
            .lines()
            .filter(|line| !line.starts_with(";") && !line.is_empty())
            .map(|line| line.split(": ").map(|part| part.to_string()).collect::<(String, String)>());

        let mut map = HashMap::with_capacity(before_validation.clone().count());
        while let Some((key, value)) = before_validation.next() {
            if map.insert(key, value).is_some() {
                return Err(HarTexError::Custom {
                    message: format!("duplicate key found in language configuration file {path}: {key}")
                })
            }
        }

        Ok(Self {
            file_map: map
        })
    }

    /// # Instance Method `Locale::lang_id`
    ///
    /// Retrieves the language identifier from the current loaded language configuration file.
    pub fn lang_id(&self) -> String {
        self.file_map.get("LanguageIdentifier").unwrap().clone()
    }
}
