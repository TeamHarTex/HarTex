/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use hartex_core::dotenv::Error as DotenvError;

#[derive(Debug)]
pub enum GatewayError {
    Dotenv { source: DotenvError }
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Dotenv { source } => write!(f, "dotenv error: {source}"),
        }
    }
}

impl Error for GatewayError {}

impl From<DotenvError> for GatewayError {
    fn from(source: DotenvError) -> Self {
        Self::Dotenv { source }
    }
}
