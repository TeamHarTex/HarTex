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
//! The `hartex_locale` crate contains translations of `HarTex` messages into various locales or
//! languages.

#![deny(clippy::pedantic, warnings)]
#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    fs,
    ops::Index
};

use hartex_base::error::{
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
    ///
    /// # Errors
    ///
    /// Returns `std::io::Error` if something wrong happens when reading the language configuration
    /// file.
    ///
    /// Returns `HarTexError::Custom` if the file language configuration file is invalid.
    #[allow(clippy::missing_panics_doc)]
    pub fn load(path: &str) -> HarTexResult<Self> {
        let file = fs::read_to_string(&path)?;

        let before_validation = file
            .lines()
            .filter(|line| !line.starts_with(';') && !line.is_empty())
            .map(|line| {
                let split = line.split_once(": ").unwrap();

                (split.0, split.1)
            })
            .collect::<Vec<_>>();

        if !before_validation
            .iter()
            .any(|entry| entry.0 == "LanguageIdentifier")
        {
            return Err(HarTexError::Custom {
                message: format!(
                    "`LanguageIdentifier` field must be specified in language configuration file: {path}"
                )
            });
        }

        let mut map = HashMap::with_capacity(before_validation.clone().len());

        for (key, value) in before_validation {
            if map.insert(key.to_string(), value.to_string()).is_some() {
                return Err(HarTexError::Custom {
                    message: format!(
                        "duplicate key found in language configuration file {path}: {key}"
                    )
                });
            }
        }

        Ok(Self {
            file_map: map
        })
    }

    /// # Instance Method `Locale::lang_id`
    ///
    /// Retrieves the language identifier from the current loaded language configuration file.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn lang_id(&self) -> String {
        self.file_map.get("LanguageIdentifier").unwrap().clone()
    }

    /// # Instance Method `Locale::get`
    ///
    /// Looks up a message with the provided key.
    ///
    /// ## Parameters
    /// - `lk_name`, type `&str`: the key of the message to look up
    #[must_use]
    pub fn get(&self, lk_name: &str) -> Option<&String> {
        self.file_map.get(lk_name)
    }
}

impl<'a> Index<&'a str> for Locale {
    type Output = String;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.get(index).expect("property not found in langcfg file")
    }
}

#[cfg(test)]
mod tests {
    use super::Locale;

    #[test]
    fn test_en_us_lang_id() {
        let locale = Locale::load("../../_langcfgs/en_GB.langcfg").unwrap();

        assert_eq!(locale.lang_id(), String::from("en_GB"));
    }
}
