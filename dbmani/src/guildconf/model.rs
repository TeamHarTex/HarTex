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

//! # The `model` Module
//!
//! This module contains some models for use in the `GetGuildConfig` future.

use std::{
    ops::Deref,
    sync::Arc
};

use hartex_base::{
    error::HarTexError,
    logging::tracing
};
use hartex_conftoml::TomlConfig;
use sqlx::{
    postgres::PgRow,
    Error,
    Result as SqlxResult,
    Row
};

pub struct GuildConfig {
    pub inner: Arc<TomlConfig>
}

impl Deref for GuildConfig {
    type Target = TomlConfig;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'r> sqlx::FromRow<'r, PgRow> for GuildConfig {
    #[allow(clippy::cast_sign_loss)]
    fn from_row(row: &'r PgRow) -> SqlxResult<Self> {
        let span = tracing::trace_span!(parent: None, "database manipulation: get guild config");
        let toml_config = row.try_get::<String, &str>("TomlConfig")?;

        let decoded = match base64::decode(toml_config) {
            Ok(bytes) => bytes,
            Err(error) => {
                let message = format!("failed to decode base64; error: `{error:?}`");

                span.in_scope(|| tracing::error!("{message}", message = &message));

                return Err(Error::Decode(Box::new(HarTexError::Base64DecodeError {
                    error
                })));
            }
        };

        span.in_scope(|| tracing::trace!("deserializing toml config..."));

        let result = hartex_conftoml::from_str(match std::str::from_utf8(&*decoded) {
            Ok(string) => string,
            Err(error) => {
                let message = format!("failed to construct utf-8 string; error: `{error:?}`");

                span.in_scope(|| tracing::error!("{message}", message = &message));

                return Err(Error::Decode(Box::new(HarTexError::Utf8ValidationError {
                    error
                })));
            }
        });

        if let Err(error) = result {
            return Err(Error::Decode(Box::new(error)));
        }

        Ok(Self {
            inner: Arc::new(result.unwrap())
        })
    }
}
