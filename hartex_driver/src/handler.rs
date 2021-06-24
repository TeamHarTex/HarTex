//! # The `handler` Module
//!
//! This module defines the `EventHandler` struct, which defines various function handlers for
//! individual events.

use hartex_core::{
    discord::model::gateway::payload::Ready,
    error::HarTexResult
};

use hartex_logging::Logger;

/// # Struct `EventHandler`
///
/// This structure defines various function handlers for individual events.
pub struct EventHandler;

impl EventHandler {
    pub async fn ready(payload: Box<Ready>) -> HarTexResult<()> {
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
}
