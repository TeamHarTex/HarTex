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

use hartex_discord_core::discord::model::application::interaction::Interaction;

/// The command metadata trait, specifying the various information about the command.
pub trait CommandMetadata {
    /// The type of the command.
    #[deprecated(since = "0.11.0")]
    fn command_type(&self) -> u8 {
        0
    }

    /// Whether the command is only available in the form of an interaction.
    #[deprecated(since = "0.11.0")]
    fn interaction_only(&self) -> bool {
        true
    }

    /// The name of the command.
    fn name(&self) -> String;
}

/// The command trait.
pub trait Command: CommandMetadata {
    /// Executes the command.
    #[allow(async_fn_in_trait)]
    async fn execute(&self, interaction: Interaction) -> miette::Result<()>;
}
