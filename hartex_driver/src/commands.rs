//! # The `commands` Module
//!
//! This module defines the command handler, which is used when a command is detected in a message.

use tokio::time;

use hartex_cmdsys::command::SlashCommand;

use hartex_core::{
    discord::http::Client,
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_logging::Logger;

pub async fn register_global_slash_commands(commands: Vec<Box<dyn SlashCommand + Send + Sync>>, http: Client) -> HarTexResult<()> {
    let mut i = 1;
    let len = commands.len();

    let _ =  match http.get_global_commands()?
        .exec()
        .await?
        .models()
        .await {
        Ok(commands) => commands,
        Err(error) => {
            Logger::error(
                format!("failed to obtain a list of existing global commands: {}", error),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            return Err(HarTexError::Custom {
                message: format!("failed to obtain a list of existing global commands: {}", error)
            });
        }
    };

    // FIXME: do not register command on every single startup
    for command in &commands {
        time::sleep(time::Duration::from_secs(1)).await;

        Logger::verbose(
            format!("registering global slash command {} of {}", i, len),
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        match http.create_global_command(&command.name(), &command.description())?
            .default_permission(command.enabled_by_default())
            .exec()
            .await {
             Ok(_) => (),
             Err(error) => {
                 Logger::error(
                     format!("failed to register global slash command {} of {}: {}", i, len, error),
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