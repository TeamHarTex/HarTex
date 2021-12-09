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

    if let Err(error) = http
        .set_global_commands(global_commands.as_slice())?
        .exec()
        .await
    {
        tracing::error!("failed to register global commands: {error}");

        return Err(HarTexError::Custom {
            message: String::from("failed to register global commands")
        });
    }

    tracing::info!("global commands registered");

    Ok(())
}
