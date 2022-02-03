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

//! Models for use in the `GetWhitelistedGuilds` future.

use std::num::NonZeroU64;

use hartex_base::discord::model::id::GuildId;
use sqlx::{
    postgres::PgRow,
    Result as SqlxResult,
    Row
};

pub struct WhitelistedGuild {
    pub GuildName: String,
    pub GuildId: GuildId
}

impl<'r> sqlx::FromRow<'r, PgRow> for WhitelistedGuild {
    #[allow(clippy::cast_sign_loss)]
    fn from_row(row: &'r PgRow) -> SqlxResult<Self> {
        let name = row.try_get::<String, &str>("GuildName")?;
        let id = row.try_get::<i64, &str>("GuildId")?;

        Ok(Self {
            GuildName: name,
            GuildId: GuildId(NonZeroU64::new(id as u64).unwrap())
        })
    }
}
