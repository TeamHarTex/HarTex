/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

//! # Entity Cache Errors

use std::env::VarError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use bb8::RunError;
use tokio_postgres::Error as PostgresError;

/// A cache error..
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum CacheError {
    Bb8(RunError<PostgresError>),
    /// Error related to environment variables.
    Env(VarError),
    /// A postgres error occurred.
    Postgres(PostgresError),
}

impl Display for CacheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bb8(error) => writeln!(f, "bb8 postgres error: {error}"),
            Self::Env(error) => writeln!(f, "env error: {error}"),
            Self::Postgres(error) => writeln!(f, "postgres error: {error}"),
        }
    }
}

impl Error for CacheError {}

impl From<RunError<PostgresError>> for CacheError {
    fn from(error: RunError<PostgresError>) -> Self {
        Self::Bb8(error)
    }
}

impl From<PostgresError> for CacheError {
    fn from(error: PostgresError) -> Self {
        Self::Postgres(error)
    }
}

impl From<VarError> for CacheError {
    fn from(error: VarError) -> Self {
        Self::Env(error)
    }
}

pub type CacheResult<T> = Result<T, CacheError>;
