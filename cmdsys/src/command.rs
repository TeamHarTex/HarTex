/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! A base command trait for commands to implement.

use hartex_base::discord::{
    cache_inmemory::CloneableInMemoryCache,
    model::application::command::{CommandOption, CommandType as TwilightCommandType},
};
use hartex_utils::FutureRetType;

use crate::context::CommandContext;

/// An application command.
#[allow(clippy::module_name_repetitions)]
pub trait Command {
    /// The name of the command.
    fn name(&self) -> String;

    /// The description of the command.
    fn description(&self) -> String;

    /// The type of the command.
    fn command_type(&self) -> CommandType;

    /// The execution function procedure.
    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        cache: CloneableInMemoryCache,
    ) -> FutureRetType<'asynchronous_trait, ()>;

    /// The required command options.
    fn required_cmdopts(&self) -> Vec<CommandOption> {
        vec![]
    }

    /// The optional command options.
    fn optional_cmdopts(&self) -> Vec<CommandOption> {
        vec![]
    }

    /// Whether the command is enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

/// The type of a command belongs to.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub enum CommandType {
    /// Slash commands; a text-based command that shows up when a user types `/`
    ChatInput,
    /// An interface-based command that shows up when you right click or tap on a message
    Message,
    /// An interface-based command that shows up when you right click or tap on a user
    User,
}

#[allow(clippy::from_over_into)]
impl Into<TwilightCommandType> for CommandType {
    fn into(self) -> TwilightCommandType {
        match self {
            Self::ChatInput => TwilightCommandType::ChatInput,
            Self::Message => TwilightCommandType::Message,
            Self::User => TwilightCommandType::User,
        }
    }
}
