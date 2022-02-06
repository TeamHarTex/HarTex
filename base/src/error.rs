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
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! An error type for error handling in the bot.

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use toml::de::Error as TomlError;

/// Various error types used within HarTex.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[non_exhaustive]
pub enum HarTexError {
    TomlError { error: TomlError }
}

impl Display for HarTexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::TomlError { error } => {
                f.write_str("toml deserialization error: ")?;
                write!(f, "{error}")?;

                if let Some((line, column)) = error.line_col() {
                    write!(f, "; at line {line} column {column}")?;
                }

                Ok(())
            },
        }
    }
}

impl Error for HarTexError {}

impl From<TomlError> for HarTexError {
    fn from(error: TomlError) -> Self {
        Self::TomlError { error }
    }
}

/// A global type-alias for handling the [`Result`] type throughout the codebase.
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
pub type HarTexResult<T> = Result<T, HarTexError>;
