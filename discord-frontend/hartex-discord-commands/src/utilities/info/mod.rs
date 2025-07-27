/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The Info Command

use async_trait::async_trait;
use hartex_discord_commands_core::{command, context::CommandContext, traits::Command};
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;

use crate::utilities::Utilities;

mod info_bot;
mod info_emoji;
mod info_role;
mod info_server;
mod info_user;

/// The `info` command declaration.
#[command(name = "info", plugin = Utilities)]
pub struct Info;

#[async_trait]
impl Command for Info {
    async fn execute(&self, context: &CommandContext<'_>) -> miette::Result<()> {
        let Some(subcommand) = context
            .command
            .options
            .iter()
            .find(|option| matches!(option.value, CommandOptionValue::SubCommand(_)))
        else {
            unreachable!()
        };

        match subcommand.name.as_str() {
            "bot" => info_bot::execute(context, subcommand).await,
            "emoji" => info_emoji::execute(context, subcommand).await,
            "role" => info_role::execute(context, subcommand).await,
            "server" => info_server::execute(context, subcommand).await,
            "user" => info_user::execute(context, subcommand).await,
            _ => unreachable!(),
        }
    }
}
