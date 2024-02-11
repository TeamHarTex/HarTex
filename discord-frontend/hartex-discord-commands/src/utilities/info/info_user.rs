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

use hartex_discord_cdn::Cdn;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::discord::util::snowflake::Snowflake;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::member::CachedMemberRepository;
use hartex_discord_entitycache_repositories::user::CachedUserRepository;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use miette::IntoDiagnostic;

#[allow(clippy::unused_async)]
pub async fn execute(interaction: Interaction, option: CommandDataOption) -> miette::Result<()> {
    let CommandOptionValue::SubCommand(options) = option.value else {
        unreachable!()
    };

    let interaction_client = CLIENT.interaction(interaction.application_id);
    let locale = interaction
        .locale
        .clone()
        .unwrap_or_else(|| String::from("en-GB"));
    let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

    let CommandOptionValue::User(user_id) = options
        .iter()
        .find(|option| option.name.as_str() == "user")
        .map_or(
            CommandOptionValue::User(interaction.author_id().unwrap()),
            |option| option.value.clone(),
        )
    else {
        unreachable!()
    };

    let user = CachedUserRepository.get(user_id).await.into_diagnostic()?;

    let userinfo_embed_generalinfo_field_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_field_name()?;
    let userinfo_embed_generalinfo_id_subfield_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_id_subfield_name()?;
    let userinfo_embed_generalinfo_name_subfield_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_name_subfield_name()?;
    let userinfo_embed_generalinfo_created_subfield_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_created_subfield_name()?;
    let userinfo_embed_serverpresence_field_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_field_name()?;
    let userinfo_embed_serverpresence_nickname_subfield_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_nickname_subfield_name()?;

    let mut builder = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            userinfo_embed_generalinfo_field_name,
            format!(
                "{} {}\n{} {}\n{} {}",
                userinfo_embed_generalinfo_id_subfield_name,
                user_id.to_string().discord_inline_code(),
                userinfo_embed_generalinfo_name_subfield_name,
                user.global_name
                    .clone()
                    .unwrap_or(String::from("<not set>")),
                userinfo_embed_generalinfo_created_subfield_name,
                (user.id.timestamp() / 1000)
                    .to_string()
                    .discord_relative_timestamp(),
            ),
        ));

    if let Some(guild_id) = interaction.guild_id {
        let member = CachedMemberRepository
            .get((guild_id, user_id))
            .await
            .into_diagnostic()?;

        builder = builder
            .field(EmbedFieldBuilder::new(
                userinfo_embed_serverpresence_field_name,
                format!(
                    "{} {}",
                    userinfo_embed_serverpresence_nickname_subfield_name,
                    member.nick.unwrap_or(String::from("<not set>")),
                ),
            ))
            .title(user.name);
    }

    builder = if let Some(avatar) = user.avatar {
        builder.thumbnail(ImageSource::url(Cdn::user_avatar(user_id, avatar)).into_diagnostic()?)
    } else if user.global_name.is_some() {
        builder.thumbnail(
            ImageSource::url(Cdn::default_user_avatar(Some(user_id), None)).into_diagnostic()?,
        )
    } else {
        builder.thumbnail(
            ImageSource::url(Cdn::default_user_avatar(None, Some(user.discriminator)))
                .into_diagnostic()?,
        )
    };

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
