/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

//! # Utility Functions for Commands

use hartex_discord_core::discord::model::application::interaction::application_command::{CommandDataOption, CommandOptionValue};

/// Extension functions for `CommandDataOption`s.
pub trait CommandDaataOptionExt {
    /// Assumes that the context of the command is a subcommand.
    /// This returns the options of the subcommand
    fn assume_subcommand(&self) -> Vec<CommandDataOption>;
}

impl CommandDaataOptionExt for CommandDataOption {
    fn assume_subcommand(&self) -> Vec<CommandDataOption> {
        let CommandOptionValue::SubCommand(options) = self.value.clone() else {
            unreachable!()
        };

        options.clone()
    }
}
