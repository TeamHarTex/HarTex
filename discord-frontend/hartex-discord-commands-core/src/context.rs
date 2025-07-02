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

use hartex_discord_core::discord::cache::DefaultInMemoryCache;
use hartex_discord_core::discord::http::client::InteractionClient;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::application::interaction::InteractionData;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandData;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::InteractionMarker;
use hartex_discord_core::discord::model::id::marker::UserMarker;
use hartex_localization_core::Localizer;
use miette::IntoDiagnostic;

/// The context of a command when ran.
pub struct CommandContext<'a> {
    pub author: Id<UserMarker>,
    pub cache: &'a DefaultInMemoryCache,
    client: &'a InteractionClient<'a>,
    pub command: Box<CommandData>,
    pub guild: Option<Id<GuildMarker>>,
    pub locale: Option<&'a String>,
    id: Id<InteractionMarker>,
    pub localizer: &'a Localizer<'a>,
    token: &'a str,
}

impl<'a> CommandContext<'a> {
    /// Construct a new command context.
    pub fn new(
        interaction: &'a Interaction,
        cache: &'a DefaultInMemoryCache,
        client: &'a InteractionClient<'a>,
        localizer: &'a Localizer<'a>,
    ) -> Self {
        let Some(InteractionData::ApplicationCommand(command)) = interaction.clone().data else {
            unreachable!()
        };

        Self {
            author: interaction.author_id().unwrap(),
            cache,
            client,
            command,
            guild: interaction.guild_id,
            id: interaction.id,
            locale: interaction.locale.as_ref(),
            localizer,
            token: &interaction.token,
        }
    }

    pub async fn create_response(&self, response: InteractionResponse) -> miette::Result<()> {
        self.client
            .create_response(self.id, self.token, &response)
            .await
            .into_diagnostic()?;

        Ok(())
    }
}
