//! # The `model` Module
//!
//! This module contains some models for use in the `GetWhitelistedGuilds` future.

use std::num::NonZeroU64;

use hartex_core::discord::model::id::GuildId;
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
