//! # The `command` Module
//!
//! This module defines a trait for commands to implement.

use hartex_core::discord::cache_inmemory::InMemoryCache;

use hartex_utils::FutureRetType;

use crate::{
    context::CommandContext,
    parser::args::CommandArgs
};

/// # Trait `Command`
///
/// A command.
///
/// ## Trait Methods
/// - `name`; return type `String`: the name of the command
/// - `execute`; parameters: `CommandContext`, `CommandArgs`, `InMemoryCache`; return type: `FutureRetType<()>`: the execution procedure
pub trait Command {
    fn name(&self) -> String;

    fn execute(ctx: CommandContext, args: CommandArgs, cache: InMemoryCache) -> FutureRetType<()>;
}

/// # Trait `SlashCommand`
///
/// A slash command.
pub trait SlashCommand {

}