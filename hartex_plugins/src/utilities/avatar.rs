/*
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

//! # The `avatar` Module
//!
//! This module implements the `avatar` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};
use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        model::application::command::{
            ChoiceCommandOptionData,
            CommandOption
        }
    },
    error::HarTexResult
};
use hartex_utils::FutureRetType;

pub struct Avatar;

impl Command for Avatar {
    fn name(&self) -> String {
        String::from("avatar")
    }

    fn description(&self) -> String {
        String::from("UtilitiesPlugin.AvatarCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        _: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_avatar_command(ctx))
    }

    fn required_cmdopts(&self) -> Vec<CommandOption> {
        vec![
            CommandOption::String(ChoiceCommandOptionData {
                autocomplete: false,
                choices: vec![],
                description: String::from("the id of the user to obtain the avatar for"),
                name: String::from("user"),
                required: true
            }),
        ]
    }
}

/// # Asynchronous Function `execute_avatar_command`
///
/// Executes the `avatar` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
#[allow(clippy::unused_async)]
async fn execute_avatar_command(_: CommandContext) -> HarTexResult<()> {
    Ok(())
}
