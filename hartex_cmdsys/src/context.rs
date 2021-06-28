//! # The `context` Module
//!
//! This module provides a command context used in commands.

use std::{
    ops::Deref,
    sync::Arc
};

pub struct CommandContext {
    inner: Arc<CommandContextInner>
}

#[derive(Clone)]
pub struct CommandContextInner {
}

impl Deref for CommandContext {
    type Target = CommandContextInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
