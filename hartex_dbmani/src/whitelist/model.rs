//! # The `model` Module
//!
//! This module contains some models for use in the `GetWhitelistedGuilds` future.

use sqlx::{
    postgres::PgRow,
    Result as SqlxResult,
    Row
};

pub struct WhitelistedGuild<'a> {
    GuildName: &'a str,
    GuildId: u64
}

impl<'a> sqlx::FromRow<'a, PgRow> for WhitelistedGuild<'a> {
    fn from_row(row: &'a PgRow) -> SqlxResult<Self> {
        let name = row.try_get::<&'a str, &'a str>("GuildName")?;
        let id = row.try_get::<i64, &'a str>("GuildId")?;

        Ok(Self {
            GuildName: name,
            GuildId: id as u64
        })
    }
}
