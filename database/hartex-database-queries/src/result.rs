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

use wtx::Error as WtxError;

#[derive(Debug)]
pub enum Error {
    Generic(&'static str),
    Wtx(WtxError),
}

impl From<WtxError> for Error {
    fn from(err: WtxError) -> Self {
        Self::Wtx(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait IntoCrateResult<T> {
    fn into_crate_result(self) -> Result<T>;
}

impl<T> IntoCrateResult<T> for wtx::Result<T> {
    fn into_crate_result(self) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::Wtx(e)),
        }
    }
}
