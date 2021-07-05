//! # The `handler` Module
//!
//! This module defines the `EventHandler` struct, which defines various function handlers for
//! individual events.

use hartex_core::{
    discord::model::gateway::{
        event::shard::Identifying,
        payload::{
            GuildCreate,
            Ready
        }
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_dbmani::whitelist::GetWhitelistedGuilds;

use hartex_logging::Logger;

/// # Struct `EventHandler`
///
/// This structure defines various function handlers for individual events.
pub struct EventHandler;

impl EventHandler {
    pub async fn guild_create(payload: Box<GuildCreate>) -> HarTexResult<()> {
        Logger::verbose(
            format!("joined a new guild with name `{}` with id {}; checking whether the guild is whitelisted", payload.name, payload.id),
            Some(module_path!())
        );

        let res = GetWhitelistedGuilds::default().await?;

        if !res.iter().any(|refmulti| {
            *refmulti == payload.id.0
        }) {
            Logger::error("guild is not whitelisted; leaving guild", Some(module_path!()));

            // TODO: add prodcedure to DM the guild owner about the whitelist status

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
