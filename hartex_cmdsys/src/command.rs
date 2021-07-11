//! # The `command` Module
//!
//! This module defines a trait for commands to implement.

use hartex_core::discord::cache_inmemory::InMemoryCache;

use hartex_utils::FutureRetType;

use crate::{
    context::CommandContext,
    parser::args::CommandArgs
};

pub trait Command {
    fn name(&self) -> String;

    fn execute(ctx: CommandContext, args: CommandArgs, cache: InMemoryCache) -> FutureRetType<()>;
}
