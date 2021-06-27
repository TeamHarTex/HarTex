//! # The `context` Module
//!
//! This module provides a command context used in commands.

use std::sync::Arc;

use hartex_core::discord::http::Client;

pub struct CommandContext<'a> {
    inner: Arc<CommandContextInner<'a>>
}

#[derive(Clone)]
pub struct CommandContextInner<'a> {
}
