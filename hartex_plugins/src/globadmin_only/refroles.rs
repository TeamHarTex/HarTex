//! # The `refroles` Module
//!
//! This module implements the `refroles` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType,
    },
    context::CommandContext
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        model::application::interaction::Interaction
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_utils::FutureRetType;

/// # Struct `Refroles`
///
/// The `refroles` command.
pub struct Refroles;

impl Command for Refroles {
    fn name(&self) -> String {
        String::from("refroles")
    }

    fn description(&self) -> String {
        String::from("GlobAdminOnlyPlugin.RefrolesCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_refroles_command(ctx))
    }
}

/// # Asynchronous Function `execute_refroles_command`
///
/// Executes the `refroles` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_refroles_command(ctx: CommandContext) -> HarTexResult<()> {
    let _ = match ctx.interaction.clone() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    Ok(())
}