//! # The `refroles` Module
//!
//! This module implements the `refroles` command.

use hartex_cmdsys::{
    checks::CheckParams,
    command::{
        Command,
        CommandType,
    },
    context::CommandContext
};

use hartex_core::{
    discord::cache_inmemory::InMemoryCache,
    error::HarTexResult
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

    fn execute_checks<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        params: CheckParams,
        checks: Box<[fn(CommandContext, CheckParams) -> FutureRetType<'asynchronous_trait, ()>]>
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(async move {
            for check in checks.iter() {
                if let Err(error) = check(ctx.clone(), params.clone()).await {
                    return Err(error);
                }

                continue;
            }

            Ok(())
        })
    }
}

/// # Asynchronous Function `execute_refroles_command`
///
/// Executes the `refroles` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_refroles_command(_: CommandContext) -> HarTexResult<()> {
    Ok(())
}