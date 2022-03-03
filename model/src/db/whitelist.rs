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

//! Guild whitelist models.

use base::discord::model::id::{marker::UserMarker, Id};
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{Error, FromRow, Row};

/// A whitelisted guild in the database.
pub struct WhitelistedGuild {
    pub(in crate) id: Id<UserMarker>,
    pub(in crate) name: String,
    pub(in crate) owner_id: Id<UserMarker>,
    pub(in crate) whitelisted_since: OffsetDateTime,
}

impl WhitelistedGuild {
    #[must_use]
    pub fn id(&self) -> Id<UserMarker> {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id
    }

    #[must_use]
    pub fn whitelisted_since(&self) -> OffsetDateTime {
        self.whitelisted_since
    }
}

impl<'r> FromRow<'r, PgRow> for WhitelistedGuild {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id_string = row.try_get::<String, &str>("id")?;
        let id = Id::new_checked(id_string.parse::<u64>().unwrap()).unwrap();
        let name = row.try_get::<String, &str>("name")?;
        let owner_id_string = row.try_get::<String, &str>("id")?;
        let owner_id = Id::new_checked(owner_id_string.parse::<u64>().unwrap()).unwrap();
        let whitelisted_since = row.try_get::<OffsetDateTime, &str>("whitelisted_since")?;

        Ok(Self {
            id,
            name,
            owner_id,
            whitelisted_since,
        })
    }
}
