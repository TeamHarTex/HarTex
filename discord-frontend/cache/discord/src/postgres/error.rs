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

use std::env::VarError;

use sqlx::Error as SqlxError;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum PostgresBackendError {
    EnvVar { src: VarError },
    Sqlx { src: SqlxError },
}

impl From<SqlxError> for PostgresBackendError {
    fn from(src: SqlxError) -> Self {
        Self::Sqlx { src }
    }
}

impl From<VarError> for PostgresBackendError {
    fn from(src: VarError) -> Self {
        Self::EnvVar { src }
    }
}
