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
    error::HarTexResult
};

use hartex_logging::Logger;

/// # Struct `EventHandler`
///
/// This structure defines various function handlers for individual events.
pub struct EventHandler;

impl EventHandler {
    pub async fn guild_create(_: Box<GuildCreate>) -> HarTexResult<()> {
        todo!()
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
