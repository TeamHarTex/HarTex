//! # The `handler` Module
//!
//! This module defines the `EventHandler` struct, which defines various function handlers for
//! individual events.

use hartex_core::{
    discord::{
        http::Client,
        model::gateway::{
            event::shard::Identifying,
            payload::{
                GuildCreate,
                Ready,
            },
        }
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_dbmani::whitelist::GetWhitelistedGuilds;

use hartex_logging::Logger;

use hartex_model::payload::CommandExecuted;

/// # Struct `EventHandler`
///
/// This structure defines various function handlers for individual events.
pub struct EventHandler;

// Twilight Events
impl EventHandler {
    /// # Static Method `EventHandler::guild_create`
    ///
    /// Handles the `GuildCreate` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<GuildCreate>`: the `GuildCreate` event payload
    /// - `http`, type `Client`: the Twilight HTTP Client to use for sending a message to the guild
    ///                          owner about his/her guild's whitelist status if the guild is not
    ///                          in the whitelis,t or that the whitelist has been removed, or that
    ///                          the guild has been previously been whitelisted but the whitelist
    ///                          is deactivated with a reason.
    pub async fn guild_create(payload: Box<GuildCreate>, http: Client) -> HarTexResult<()> {
        let guild_id = payload.id;

        Logger::verbose(
            format!("joined a new guild with name `{}` with id {}; checking whether the guild is whitelisted", payload.name, guild_id.0),
            Some(module_path!())
        );

        let res = GetWhitelistedGuilds::default().await?;

        if !res.iter().any(|refmulti| {
            *refmulti == guild_id.0
        }) {
            Logger::error("guild is not whitelisted", Some(module_path!()));

            let guild = http.guild(guild_id).await?;

            Logger::verbose("dming guild owner about the whitelist status", Some(module_path!()));

            if guild.is_none() {
                Logger::error(format!("failed to retrieve guild from its id: {}", guild_id), Some(module_path!()));

                return Err(HarTexError::Custom {
                    message: format!("failed to retrieve guild from its id: {}", guild_id)
                });
            }

            let guild_unwrap = guild.unwrap();
            // it is OK to call `.unwrap()` here as thorough checking has been done before reaching
            // this code
            let guild_owner = guild_unwrap.owner_id;

            let user = http.user(guild_owner).await?;

            if user.is_none() {
                Logger::error(format!("failed to retrieve guild owner from his/her user id: {}", guild_owner), Some(module_path!()));

                return Err(HarTexError::Custom {
                    message: format!("failed to retrieve guild owner from his/her user id: {}", guild_owner)
                });
            }

            let dm_channel = http.create_private_channel(user.unwrap().id).await?;
            let message = "Hey there! It looks like you added HarTex to your guild by the name of \"".to_string()
                + &guild_unwrap.name + "\".\n\n"
                + "Unfortunately, your guild has not been whitelisted yet and the bot cannot be "
                + "invited to your guild until you apply for a whitelist and that the application is "
                + "accepted.\n\n"
                + "You may apply for a guild whitelist if your guild meets the following criteria, they include, but not limited to:\n"
                + " - guild member count of at least 250;"
                + " - be always abide by the Discord Terms of Service (<https://discord.com/terms>) and Community Guidelines (<https://discord.com/guidelines);"
                + " - how old is the guild and/or how active is it; and"
                + " - your experience level with TOML to configure the bot before using it.\n\n"
                + "You may join our Support Guild at <discord.gg/s8qjxZK> for more information, including the application link in which you may use"
                + "to apply for a whitelist application. Good luck!";

            http.create_message(dm_channel.id).content(message)?.await?;

            Logger::error("leaving guild", Some(module_path!()));

            http.leave_guild(guild_id).await?;

            return Err(HarTexError::Custom {
                message: String::from("guild is not whitelisted")
            });
        }

        Logger::info("guild is whitelisted", Some(module_path!()));

        Ok(())
    }

    /// # Static Method `EventHandler::ready`
    ///
    /// Handles the `Ready` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<Ready>`: the `Ready` event payload
    pub async fn ready(payload: Box<Ready>) -> HarTexResult< ()> {
        let user = payload.user;

        Logger::info(
            format!(
                "{}#{} [id: {}] has successfully startup; using discord api v{}",
                user.name,
                user.discriminator,
                user.id,
                payload.version
            ),
            Some(module_path!())
        );

        Ok(())
    }

    /// # Static Method `EventHandler::shard_identifying`
    ///
    /// Handles the `Identifying` event.
    ///
    /// ## Parameters
    ///
    /// - `payload`, type `Identifying`: the `Identifying` event payload
    pub async fn shard_identifying(payload: Identifying) -> HarTexResult<()> {
        Logger::verbose(
            format!(
                "shard {} is identifying with the discord gateway",
                payload.shard_id
            ),
            Some(module_path!())
        );

        Ok(())
    }
}

// Custom Events
impl EventHandler {
    pub async fn command_executed(payload: Box<CommandExecuted>) -> HarTexResult<()> {
        Logger::info(
            format!("command `{}` is executed in guild {}", payload.command, payload.guild.name),
            Some(module_path!())
        );

        Ok(())
    }
}
