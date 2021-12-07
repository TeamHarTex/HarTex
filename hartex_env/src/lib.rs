/*
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # `hartex_env` - Environment Wrapper for `HarTex` Discord bot
//!
//! The `hartex_env` library various environments for different aspects of the `HarTex` Discord bot.

#![deny(clippy::pedantic, warnings, unsafe_code)]

use std::{
    env,
    num::NonZeroU64
};

use hartex_core::{
    discord::model::id::{
        GuildId,
        RoleId,
        UserId
    },
    logging::tracing
};

/// # Struct `DatabaseEnv`
///
/// Represents a collection of environment variables useful for the bot during database
/// manipulations.
pub struct DatabaseEnv {
    pub pgsql_credentials_guilds: Option<String>,
    pub pgsql_credentials_guildconfig: Option<String>
}

impl DatabaseEnv {
    /// # Static Method `DatabaseEnv::get`
    ///
    /// Retrieves the environment variables.
    #[must_use]
    pub fn get() -> Self {
        tracing::trace!("retrieving `PGSQL_CREDENTIALS_GUILDS` environment variable");
        let pgsql_credentials_guilds = env::var("PGSQL_CREDENTIALS_GUILDS").ok();

        tracing::trace!("retrieving `PGSQL_CREDENTIALS_GUILDCONFIG` environment variable");
        let pgsql_credentials_guildconfig = env::var("PGSQL_CREDENTIALS_GUILDCONFIG").ok();

        Self {
            pgsql_credentials_guilds,
            pgsql_credentials_guildconfig
        }
    }
}

/// # Struct `PluginEnv`
///
/// Represents a collection of environment variables useful for the bot in plugins.
pub struct PluginEnv {
    pub global_administrator_uid: Option<UserId>,
    pub support_guild_gid: Option<GuildId>,
    pub hartex_guild_owner_rid: Option<RoleId>,
    pub hartex_user_rid: Option<RoleId>
}

impl PluginEnv {
    /// # Static Method `PluginEnv::get`
    ///
    /// Retrieves the environment variables.
    #[allow(clippy::missing_panics_doc)] // this function should never panic
    #[must_use]
    pub fn get() -> Self {
        tracing::trace!("retrieving `GLOBAL_ADMINISTRATOR_UID` environment variable");
        let global_administrator_uid = env::var("GLOBAL_ADMINISTRATOR_UID").ok();

        tracing::trace!("retrieving `SUPPORT_GUILD_GID` environment variable");
        let support_guild_gid = env::var("SUPPORT_GUILD_GID").ok();

        tracing::trace!("retrieving `HARTEX_GUILD_OWNER_RID` environment variable");
        let hartex_guild_owner_rid = env::var("HARTEX_GUILD_OWNER_RID").ok();

        tracing::trace!("retrieving `HARTEX_USER_RID` environment variable");
        let hartex_user_rid = env::var("HARTEX_USER_RID").ok();

        Self {
            global_administrator_uid: global_administrator_uid
                .map(|uid| UserId(NonZeroU64::new(uid.parse().unwrap()).unwrap())),
            support_guild_gid: support_guild_gid
                .map(|gid| GuildId(NonZeroU64::new(gid.parse().unwrap()).unwrap())),
            hartex_guild_owner_rid: hartex_guild_owner_rid
                .map(|rid| RoleId(NonZeroU64::new(rid.parse().unwrap()).unwrap())),
            hartex_user_rid: hartex_user_rid
                .map(|rid| RoleId(NonZeroU64::new(rid.parse().unwrap()).unwrap()))
        }
    }
}

/// # Struct `StartupEnv`
///
/// Represents a collection of environment variables useful for the bot during startup.
pub struct StartupEnv {
    pub application_aid: Option<NonZeroU64>,
    pub bot_token: Option<String>
}

impl StartupEnv {
    /// # Static Method `StartupEnv::get`
    ///
    /// Retrieves the environment variables.
    #[allow(clippy::missing_panics_doc)] // this function should never panic
    #[must_use]
    pub fn get() -> Self {
        tracing::trace!("retrieving `APPLICATION_AID` environment variable");
        let application_aid = env::var("APPLICATION_AID").ok();

        tracing::trace!("retrieving `BOT_TOKEN` environment variable");
        let bot_token = env::var("BOT_TOKEN").ok();

        Self {
            application_aid: application_aid
                .map(|id| NonZeroU64::new(id.parse().unwrap()).unwrap()),
            bot_token
        }
    }
}
