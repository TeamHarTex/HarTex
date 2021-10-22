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
    model::application::interaction::Interaction
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
pub struct CommandContextInner {
    pub http: Arc<Client>,
    pub cluster: Arc<Cluster>,
    pub interaction: Interaction
}

impl Deref for CommandContext {
    type Target = CommandContextInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
