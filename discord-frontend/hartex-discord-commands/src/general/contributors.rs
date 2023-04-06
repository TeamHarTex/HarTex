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

use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedAuthorBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFooterBuilder;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(name = "contributors")]
pub struct Contributors;

impl Command for Contributors {
    async fn execute(&self, interaction: Interaction) -> hartex_eyre::Result<()> {
        let interaction_client = CLIENT.interaction(interaction.application_id);
        let bundle = create_bundle(
            interaction.locale.and_then(|locale| locale.parse().ok()),
            &["discord-frontend", "commands"],
        )?;

        bundle_get!(bundle."contributors-embed-title": message, out [contributors_embed_title, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-description": message, out [contributors_embed_description, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-global-admin-field-name": message, out [contributors_embed_global_admin_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-front-dev-field-name": message, out [contributors_embed_front_dev_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-translation-team-field-name": message, out [contributors_embed_translation_team_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-footer": message, out [contributors_embed_footer, errors]);
        handle_errors(errors)?;

        let embed = EmbedBuilder::new()
            .author(EmbedAuthorBuilder::new(contributors_embed_title).build())
            .description(contributors_embed_description)
            .field(
                EmbedFieldBuilder::new(
                    contributors_embed_global_admin_field_name,
                    "HTGAzureX1212.#4937",
                )
                .build(),
            )
            .field(
                EmbedFieldBuilder::new(contributors_embed_front_dev_field_name, "Ariz#0288")
                    .build(),
            )
            .field(
                EmbedFieldBuilder::new(
                    contributors_embed_translation_team_field_name,
                    "teddy#6071 (Locale: `zh-CN`)\n星曌#4316 (Locale: `zh-TW`)",
                )
                .build(),
            )
            .footer(EmbedFooterBuilder::new(contributors_embed_footer))
            .validate()?
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
            .await?;

        Ok(())
    }
}
