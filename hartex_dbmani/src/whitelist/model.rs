//! # The `model` Module
//!
//! This module contains some models for use in the `GetWhitelistedGuilds` future.

use sqlx::{
    postgres::PgRow,
    Result as SqlxResult,
    Row
};

pub struct WhitelistedGuild {
    pub GuildName: String,
    pub GuildId: u64
}

impl<'r> sqlx::FromRow<'r, PgRow> for WhitelistedGuild {
    fn from_row(row: &'r PgRow) -> SqlxResult<Self> {
        let name = row.try_get::<String, &str>("GuildName")?;
        let id = row.try_get::<i64, &str>("GuildId")?;

        Ok(Self {
            GuildName: name,
            GuildId: id as u64
        })
    }
}
