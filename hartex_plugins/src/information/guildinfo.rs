//! # The `guildinfo` Module
//!
//! This module implements the `guildinfo` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType,
    },
    context::CommandContext
};

use hartex_core::discord::{
    cache_inmemory::InMemoryCache,
    model::application::command::{
        ChoiceCommandOptionData,
        CommandOption
    }
};

use hartex_utils::FutureRetType;

/// # Struct `Guildinfo`
///
/// The `guildinfo` command.
pub struct Guildinfo;

impl Command for Guildinfo {
    fn name(&self) -> String {
        String::from("guildinfo")
    }

    fn description(&self) -> String {
        String::from("InformationPlugin.GuildinfoCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(&self, _: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        todo!()
    }
}