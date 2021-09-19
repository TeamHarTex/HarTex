//! # The `commands` Module
//!
//! This module defines the command handler, which is used when a command is detected in a message.

use tokio::time;

use hartex_cmdsys::command::{
    Command,
    CommandType
};

use hartex_core::{
    discord::http::Client,
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_logging::Logger;

/// # Asynchronous Function `register_global_commands`
///
/// Registers a global slash command if it has not been previously added.
///
/// ## Parameters
/// `commands`, type `Vec<Box<dyn SlashCommand + Send + Sync>>`: the commands to register.
/// `http`, type `Client`: the Twilight HTTP client to use for registration.
pub async fn register_global_commands(commands: Vec<Box<dyn Command + Send + Sync>>, http: Client) -> HarTexResult<()> {
    let mut i = 1;
    let len = commands.len();

    let existing =  match http.get_global_commands()?
        .exec()
        .await?
        .models()
        .await {
        Ok(commands) => commands,
        Err(error) => {
            Logger::error(
                format!("failed to obtain a list of existing global commands: {error}"),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            return Err(HarTexError::Custom {
                message: format!("failed to obtain a list of existing global commands: {error}")
            });
        }
    };

    let names = existing.iter().map(|command| command.name.clone()).collect::<Vec<_>>();

    for command in &commands {
        Logger::verbose(
            format!(
                "registering global command {i} of {len}; [name: {name}, type: {command_type:?}]",
                name = &command.name(),
                command_type = &command.command_type()
            ),
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        if names.contains(&command.name()) {
            Logger::verbose(
                "command already registered, skipping",
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            i += 1;

            continue;
        }

        time::sleep(time::Duration::from_secs(1)).await;

        let name = command.name();
        let create_global_command = http.new_create_global_command(&name)?;

        match {
            match command.command_type() {
                CommandType::ChatInput => {
                    create_global_command
                        .chat_input(&command.description())?
                        .command_options(&command.required_cmdopts())?
                        .command_options(&command.optional_cmdopts())?
                        .default_permission(command.enabled_by_default())
                        .exec()
                },
                CommandType::Message => todo!(),
                CommandType::User => todo!()
            }
        }.await {
            Ok(_) => (),
            Err(error) => {
                Logger::error(
                    format!("failed to register global command {i} of {len}: {error}"),
                    Some(module_path!()),
                    file!(),
                    line!(),
                    column!()
                );
            }
        }

        i += 1;
    }

    Ok(())
}