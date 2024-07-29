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

use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_core::discord::model::id::marker::UserMarker;
use hartex_discord_core::discord::model::id::Id;

/// Extension functions for `CommandDataOption`s.
pub trait CommandDataOptionExt {
    /// Assumes that the context of the command is a subcommand.
    /// This returns the options of the subcommand
    fn assume_subcommand(&self) -> Vec<CommandDataOption>;
}

impl CommandDataOptionExt for CommandDataOption {
    fn assume_subcommand(&self) -> Vec<CommandDataOption> {
        let CommandOptionValue::SubCommand(options) = self.value.clone() else {
            unreachable!()
        };

        options.clone()
    }
}

/// Extension functions for collections of `CommandDataOption`s.
pub trait CommandDataOptionsExt {
    /// Returns the value of a boolean option from a collection of options.
    fn boolean_value_of(&self, name: &str) -> bool;

    /// Returns the value of a role option from a collection of options.
    fn role_value_of(&self, name: &str) -> Id<RoleMarker>;

    /// Returns the value of a string option from a collection of options.
    fn string_value_of(&self, name: &str) -> String;

    /// Returns the value of a user option from a collection of options.
    fn user_value_of(&self, name: &str) -> Id<UserMarker>;
}

impl CommandDataOptionsExt for Vec<CommandDataOption> {
    fn boolean_value_of(&self, name: &str) -> bool {
        let CommandOptionValue::Boolean(boolean) = self
            .iter()
            .find(|option| option.name.as_str() == name)
            .map_or(CommandOptionValue::Boolean(false), |option| {
                option.value.clone()
            })
        else {
            unreachable!()
        };

        boolean
    }

    fn role_value_of(&self, name: &str) -> Id<RoleMarker> {
        let CommandOptionValue::Role(role_id) = self
            .iter()
            .find(|option| option.name.as_str() == name)
            .map(|option| option.value.clone())
            .unwrap()
        else {
            unreachable!();
        };

        role_id
    }

    fn string_value_of(&self, name: &str) -> String {
        let CommandOptionValue::String(option) = self
            .iter()
            .find(|option| option.name.as_str() == name)
            .map_or(CommandOptionValue::String(String::new()), |option| {
                option.value.clone()
            })
        else {
            unreachable!()
        };

        option
    }

    fn user_value_of(&self, name: &str) -> Id<UserMarker> {
        let CommandOptionValue::User(user_id) = self
            .iter()
            .find(|option| option.name.as_str() == name)
            .map(|option| option.value.clone())
            .unwrap()
        else {
            unreachable!();
        };

        user_id
    }
}
