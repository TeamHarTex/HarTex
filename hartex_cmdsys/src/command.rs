//! # The `command` Module
//!
//! This module defines a trait for commands to implement.

use hartex_core::discord::{
    cache_inmemory::InMemoryCache,
    model::application::command::CommandOption
};

use hartex_utils::FutureRetType;

use crate::context::CommandContext;

/// # Trait `Command`
///
/// An application command.
///
/// ## Trait Methods
/// - `name`; return type `String`: the name of the command
/// - `description`; return type `String`: the description of the command
/// - `execute`; parameters `CommandContext`, `InMemoryCache`; return type `FutureRetType<()>`: the execution procedure
/// - `execute_checks`; parameters `CommandContext`, `CheckParams`, `Box<[fn(CommandContext, CheckParams) -> FutureRetType<'asynchronous_trait, ()>]>`; return type `FutureRetType<()>`: execute checks before running a command
/// - `required_cmdopts`; return type `Vec<CommandOption>`: a vector of required command options
/// - `optional_cmdopts`; return type `Vec<CommandOption>`: a vector of optional command options
/// - `enabled_by_default`; return type `bool`: whether the slash command is enabled by default when added to a guild
pub trait Command {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn command_type(&self) -> CommandType;

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, cache: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()>;

    fn required_cmdopts(&self) -> Vec<CommandOption> {
        vec![]
    }

    fn optional_cmdopts(&self) -> Vec<CommandOption> {
        vec![]
    }

    fn enabled_by_default(&self) -> bool {
        true
    }
}

/// # Enumeration `CommandType`
///
/// Represents the type of a command belongs to.
#[derive(Clone, Debug)]
pub enum CommandType {
    /// # Enumeration Variant `CommandType::ChatInput`
    ///
    /// Slash commands; a text-based command that shows up when a user types `/`
    ChatInput,

    /// # Enumeration Variant `CommandType::Message`
    ///
    /// An interface-based command that shows up when you right click or tap on a message
    Message,

    /// # Enumeration Variant `CommandType::User`
    ///
    /// An interface-based command that shows up when you right click or tap on a user
    User
}