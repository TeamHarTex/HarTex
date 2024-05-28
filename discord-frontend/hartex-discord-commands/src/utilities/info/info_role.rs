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

//! # The Info Role Subcommand
//!
//! This command returns informatiomn about a role.

use hartex_discord_cdn::Cdn;
use hartex_discord_core::discord::mention::Mention;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::discord::util::snowflake::Snowflake;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::role::CachedRoleRepository;
use hartex_discord_utils::commands::CommandDataOptionExt;
use hartex_discord_utils::commands::CommandDataOptionsExt;
use hartex_discord_utils::localizable::Localizable;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_localization_core::Localizer;
use miette::IntoDiagnostic;
use hartex_discord_core::discord::http::client::InteractionClient;

/// Executes the `info emoji` command.
pub async fn execute(
    interaction: Interaction,
    interaction_client: &InteractionClient,
    option: CommandDataOption,
    localizer: Localizer,
) -> miette::Result<()> {
    let options = option.assume_subcommand();

    let langid_locale = interaction
        .locale
        .clone()
        .and_then(|locale| locale.parse().ok());

    let role_id = options.role_value_of("role");

    let roleinfo_embed_generalinfo_field_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_field_name()?;
    let roleinfo_embed_generalinfo_id_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_id_subfield_name()?;
    let roleinfo_embed_generalinfo_created_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_created_subfield_name()?;
    let roleinfo_embed_generalinfo_color_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_generalinfo_color_subfield_name()?;
    let roleinfo_embed_description =
        localizer.utilities_plugin_roleinfo_embed_description(role_id.mention().to_string())?;
    let roleinfo_embed_attributes_field_name =
        localizer.utilities_plugin_roleinfo_embed_attributes_field_name()?;
    let roleinfo_embed_attributes_hoist_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_attributes_hoist_subfield_name()?;
    let roleinfo_embed_attributes_managed_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_attributes_managed_subfield_name()?;
    let roleinfo_embed_attributes_mentionable_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_attributes_mentionable_subfield_name()?;
    let roleinfo_embed_attributes_position_subfield_name =
        localizer.utilities_plugin_roleinfo_embed_attributes_position_subfield_name()?;

    let role = CachedRoleRepository
        .get((interaction.guild_id.unwrap(), role_id))
        .await
        .into_diagnostic()?;

    let mut builder = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .description(roleinfo_embed_description)
        .field(EmbedFieldBuilder::new(
            format!("<:role:1139004530277765211> {roleinfo_embed_generalinfo_field_name}"),
            format!(
                "{} {}\n{} {}\n{} `#{:06X}`",
                roleinfo_embed_generalinfo_id_subfield_name,
                role.id.to_string().discord_inline_code(),
                roleinfo_embed_generalinfo_created_subfield_name,
                (role.id.timestamp() / 1000)
                    .to_string()
                    .discord_relative_timestamp(),
                roleinfo_embed_generalinfo_color_subfield_name,
                role.color,
            ),
        ))
        .field(EmbedFieldBuilder::new(
            roleinfo_embed_attributes_field_name,
            format!(
                "{} {}\n{} {}\n{} {}\n{} {}",
                roleinfo_embed_attributes_hoist_subfield_name,
                role.hoist.localize(langid_locale.clone())?,
                roleinfo_embed_attributes_managed_subfield_name,
                role.managed.localize(langid_locale.clone())?,
                roleinfo_embed_attributes_mentionable_subfield_name,
                role.mentionable.localize(langid_locale)?,
                roleinfo_embed_attributes_position_subfield_name,
                role.position,
            ),
        ));

    if let Some(icon) = role.icon {
        builder =
            builder.thumbnail(ImageSource::url(Cdn::role_icon(role.id, icon)).into_diagnostic()?);
    }

    let embed = builder.validate().into_diagnostic()?.build();

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
