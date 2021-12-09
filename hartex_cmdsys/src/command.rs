/* SPDX-License-Identifier: GPL-2.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # The `command` Module
//!
//! This module defines a trait for commands to implement.

use hartex_core::discord::{
    cache_inmemory::CloneableInMemoryCache,
    model::application::command::{
        CommandOption,
        CommandType as TwilightCommandType
    }
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
#[allow(clippy::module_name_repetitions)]
pub trait Command {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn command_type(&self) -> CommandType;

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        cache: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()>;

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
#[allow(clippy::module_name_repetitions)]
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

#[allow(clippy::from_over_into)]
impl Into<TwilightCommandType> for CommandType {
    fn into(self) -> TwilightCommandType {
        match self {
            Self::ChatInput => TwilightCommandType::ChatInput,
            Self::Message => TwilightCommandType::Message,
            Self::User => TwilightCommandType::User
        }
    }
}
