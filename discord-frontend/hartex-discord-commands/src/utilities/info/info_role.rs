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

use hartex_discord_core::discord::mention::Mention;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::role::CachedRoleRepository;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use miette::IntoDiagnostic;

pub async fn execute(interaction: Interaction, option: CommandDataOption) -> miette::Result<()> {
    let CommandOptionValue::SubCommand(options) = option.value else {
        unreachable!()
    };

    let interaction_client = CLIENT.interaction(interaction.application_id);
    let locale = interaction.locale.unwrap_or_else(|| String::from("en-GB"));
    let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

    let CommandOptionValue::Role(role_id) = options
        .iter()
        .find(|option| option.name.as_str() == "role")
        .map(|option| option.value.clone())
        .unwrap()
    else {
        unreachable!();
    };

    let roleinfo_embed_generalinfo_field_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_field_name()?;
    let roleinfo_embed_generalinfo_id_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_id_subfield_name()?;
    let roleinfo_embed_generalinfo_color_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_color_subfield_name()?;
    let roleinfo_embed_description =
        localizer.utilities_plugin_roleinfo_embed_description(role_id.mention().to_string())?;
    let roleinfo_embed_attributes_field_name =
        localizer.utilities_plugin_roleinfo_embed_attributes_field_name()?;

    let role = CachedRoleRepository
        .get((interaction.guild_id.unwrap(), role_id))
        .await
        .into_diagnostic()?;
    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .description(roleinfo_embed_description)
        .field(EmbedFieldBuilder::new(
            format!("<:role:1139004530277765211> {roleinfo_embed_generalinfo_field_name}"),
            format!(
                "{} {}\n{} `#{:06X}`",
                roleinfo_embed_generalinfo_id_subfield_name,
                role.id.to_string().discord_inline_code(),
                roleinfo_embed_generalinfo_color_subfield_name,
                role.color,
            ),
        ))
        .field(EmbedFieldBuilder::new(
            format!("{roleinfo_embed_attributes_field_name}"),
            ""
        ))
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
