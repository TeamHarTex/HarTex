//! # The `context` Module
//!
//! This module provides a command context used in commands.

use std::{
    ops::Deref,
    sync::Arc
};

use hartex_core::discord::{
    gateway::CloneableCluster,
    http::CloneableClient,
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
    pub http: CloneableClient,
    pub cluster: CloneableCluster,
    pub interaction: Interaction
}

impl Deref for CommandContext {
    type Target = CommandContextInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
