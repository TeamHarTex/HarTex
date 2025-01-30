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

use std::io;

use sqlparser::parser::ParserError;
use syn::Error as SynError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    QueryFile(&'static str),
    Sql(ParserError),
    Syn(SynError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Self::Io(err)
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Error {
        Self::Sql(err)
    }
}

impl From<SynError> for Error {
    fn from(err: SynError) -> Error {
        Self::Syn(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
