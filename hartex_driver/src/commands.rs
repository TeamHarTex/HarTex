//! # The `commands` Module
//!
//! This module defines the command handler, which is used when a command is detected in a message.

use tokio::time;

use hartex_cmdsys::{
    command::{
        Command,
        SlashCommand
    },
    context::CommandContext,
    parser::ParsedCommand
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        http::Client
    },
    error::HarTexResult
};

use hartex_eventsys::{
    emitter::EventEmitter
};

use hartex_logging::Logger;

use hartex_plugins::global::{
    about::About,
    ping::Ping,
    source::Source,
    team::Team
};

pub async fn handle_command(
    command: ParsedCommand<'_>,
    _: EventEmitter,
    cache: InMemoryCache,
    context: CommandContext
) -> HarTexResult<()> {
    Logger::verbose(
        "command identified, executing command",
        Some(module_path!()),
        file!(),
        line!(),
        column!()
    );

    match command {
        ParsedCommand { name: "about", args } => {
            About.execute_command(context, args, cache).await?;
        }
        ParsedCommand { name: "ping", args} => {
            Ping.execute_command(context, args, cache).await?;
        }
        ParsedCommand { name: "source", args} => {
            Source.execute_command(context, args, cache).await?;
        }
        ParsedCommand { name: "team", args } => {
            Team.execute_command(context, args, cache).await?;
        }
        _ => ()
    }

    Ok(())
}

pub async fn register_global_slash_commands(commands: Vec<Box<dyn CommandSlashCommand + Send + Sync>>, http: Client) -> HarTexResult<()> {
    let mut i = 1;

    for command in &commands {
        Logger::verbose(
            format!("registering global slash command {} of {}", i, commands.len()),
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
                     format!("failed to register global slash command {} of {}: {}", i, commands.len(), error),
                     Some(module_path!()),
                     file!(),
                     line!(),
                     column!()
                 );
             }
         }

        i += 1;

        time::sleep(time::Duration::from_secs(1)).await;
    }

    Ok(())
}

pub trait CommandSlashCommand: Command + SlashCommand { }

impl<T> CommandSlashCommand for T
where
    T: Command + SlashCommand { }