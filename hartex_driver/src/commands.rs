//! # The `commands` Module
//!
//! This module defines the command handler, which is used when a command is detected in a message.

use hartex_cmdsys::command::Command;
use hartex_core::{
    discord::{
        http::CloneableClient,
        util::builder::command::CommandBuilder
    },
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing
};

/// # Asynchronous Function `register_global_commands`
///
/// Registers a global slash command if it has not been previously added.
///
/// ## Parameters
/// `commands`, type `Vec<Box<dyn SlashCommand + Send + Sync>>`: the commands to register.
/// `http`, type `Client`: the Twilight HTTP client to use for registration.
///
/// ## Errors
///
/// Returns various errors when the procedure fails.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::needless_collect)]
pub async fn register_global_commands(
    commands: Vec<Box<dyn Command + Send + Sync>>,
    http: CloneableClient
) -> HarTexResult<()> {
    tracing::trace!("registering global commands");

    let global_commands = commands
        .into_iter()
        .map(|command| {
            let mut builder = CommandBuilder::new(
                command.name(),
                command.description(),
                command.command_type().into()
            )
            .default_permission(command.enabled_by_default());

            command.required_cmdopts().iter().for_each(|option| {
                let temp = builder.clone().option(option.clone());

                builder = temp;
            });

            command.optional_cmdopts().iter().for_each(|option| {
                let temp = builder.clone().option(option.clone());

                builder = temp;
            });

            builder.build()
        })
        .collect::<Vec<_>>();

    if let Err(error) = http.set_global_commands(global_commands.as_slice())?
        .exec()
        .await {
        tracing::error!("failed to register global commands: {error}");

        return Err(HarTexError::Custom {
            message: String::from("failed to register global commands")
        });
    }

    tracing::info!("global commands registered");

    Ok(())
}
