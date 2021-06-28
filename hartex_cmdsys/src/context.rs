//! # The `context` Module
//!
//! This module provides a command context used in commands.

use std::sync::Arc;

pub struct CommandContext {
    inner: Arc<CommandContextInner>
}

#[derive(Clone)]
pub struct CommandContextInner {
}
