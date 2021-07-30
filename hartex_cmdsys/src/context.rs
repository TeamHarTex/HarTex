//! # The `context` Module
//!
//! This module provides a command context used in commands.

use std::{
    ops::Deref,
    sync::Arc
};

use hartex_core::discord::{
    gateway::Cluster,
    http::Client,
    model::{
        application::interaction::Interaction,
        channel::Message
    },
};

/// # Struct `CommandContext`
///
/// The command context used for command invocation.
#[derive(Clone)]
pub struct CommandContext {
    pub inner: Arc<CommandContextInner>
}

/// # Struct `CommandContextInner`
///
/// The inner structure for `CommandContext`.
#[derive(Clone)]
#[non_exhaustive]
pub struct CommandContextInner {
    pub http: Client,
    pub message: Message,
    pub cluster: Cluster,
    pub interaction: Option<Interaction>
}

impl Deref for CommandContext {
    type Target = CommandContextInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
