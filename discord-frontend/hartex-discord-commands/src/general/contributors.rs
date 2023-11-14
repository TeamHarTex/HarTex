/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

//! # The Contributors Command

use hartex_discord_commands_core::metadata;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedAuthorBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFooterBuilder;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_localization_core::LOCALIZATION_HOLDER;
use hartex_localization_core::Localizer;
use hartex_discord_utils::CLIENT;
use miette::IntoDiagnostic;

#[metadata(command_type = 1, interaction_only = true, name = "contributors")]
pub struct Contributors;

impl Command for Contributors {
    async fn execute(&self, interaction: Interaction) -> miette::Result<()> {
        let interaction_client = CLIENT.interaction(interaction.application_id);
        let locale = interaction.locale.unwrap_or_else(|| String::from("en-GB"));
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        let contributors_embed_title = localizer.general_plugin_contributors_embed_title()?;
        let contributors_embed_description =
            localizer.general_plugin_contributors_embed_description()?;
        let contributors_embed_global_admin_field_name =
            localizer.general_plugin_contributors_embed_global_admin_field_name()?;
        let contributors_embed_front_dev_field_name =
            localizer.general_plugin_contributors_embed_front_dev_field_name()?;
        let contributors_embed_translation_team_field_name =
            localizer.general_plugin_contributors_embed_translation_team_field_name()?;
        let contributors_embed_footer = localizer.general_plugin_contributors_embed_footer()?;

        let embed = EmbedBuilder::new()
            .author(EmbedAuthorBuilder::new(contributors_embed_title).build())
            .color(0x41_A0_DE)
            .description(contributors_embed_description)
            .field(
                EmbedFieldBuilder::new(
                    contributors_embed_global_admin_field_name,
                    "htgazurex1212.",
                )
                .build(),
            )
            .field(
                EmbedFieldBuilder::new(contributors_embed_front_dev_field_name, "arizlunari")
                    .build(),
            )
            .field(
                EmbedFieldBuilder::new(
                    contributors_embed_translation_team_field_name,
                    "madonuko (Locale: `ja`)\nteddyji (Locale: `zh-CN`)\nxzihnago (Locale: `zh-TW`)",
                )
                .build(),
            )
            .footer(EmbedFooterBuilder::new(contributors_embed_footer))
            .validate()
            .into_diagnostic()?
            .build();

        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .embeds(vec![embed])
                            .build(),
                    ),
                },
            )
            .await
            .into_diagnostic()?;

        Ok(())
    }
}
