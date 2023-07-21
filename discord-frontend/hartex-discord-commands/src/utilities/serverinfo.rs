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

use std::borrow::Cow;

use hartex_discord_cdn::Cdn;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::mention::Mention;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::channel::ChannelType;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::discord::util::snowflake::Snowflake;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::guild::CachedGuildRepository;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;
use miette::IntoDiagnostic;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(minimum_level = 0)]
#[metadata(name = "serverinfo")]
pub struct ServerInfo;

impl Command for ServerInfo {
    async fn execute(&self, interaction: Interaction) -> miette::Result<()> {
        let interaction_client = CLIENT.interaction(interaction.application_id);
        let bundle = create_bundle(
            interaction.locale.and_then(|locale| locale.parse().ok()),
            &["discord-frontend", "commands"],
        )?;

        let guild = CachedGuildRepository
            .get(interaction.guild_id.unwrap())
            .await
            .into_diagnostic()?;

        bundle_get!(bundle."serverinfo-embed-generalinfo-field-name": message, out [serverinfo_embed_generalinfo_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-generalinfo-id-subfield-name": message, out [serverinfo_embed_generalinfo_id_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-generalinfo-created-subfield-name": message, out [serverinfo_embed_generalinfo_created_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-generalinfo-owner-subfield-name": message, out [serverinfo_embed_generalinfo_owner_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-generalinfo-enabled-features-subfield-name": message, out [serverinfo_embed_generalinfo_enabled_features_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-channelinfo-field-name": message, out [serverinfo_embed_channelinfo_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-channelinfo-categories-subfield-name": message, out [serverinfo_embed_channelinfo_categories_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-channelinfo-textchannels-subfield-name": message, out [serverinfo_embed_channelinfo_textchannels_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-channelinfo-voicechannels-subfield-name": message, out [serverinfo_embed_channelinfo_voicechannels_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-channelinfo-announcementchannels-subfield-name": message, out [serverinfo_embed_channelinfo_announcementchannels_subfield_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-channelinfo-stagechannels-subfield-name": message, out [serverinfo_embed_channelinfo_stagechannels_subfield_name, errors]);
        handle_errors(errors)?;

        let channels = CLIENT
            .guild_channels(guild.id)
            .await
            .into_diagnostic()?
            .model()
            .await
            .into_diagnostic()?;
        let category_count = channels
            .iter()
            .filter(|channel| channel.kind == ChannelType::GuildCategory)
            .count();
        let text_count = channels
            .iter()
            .filter(|channel| channel.kind == ChannelType::GuildText)
            .count();
        let voice_count = channels
            .iter()
            .filter(|channel| channel.kind == ChannelType::GuildVoice)
            .count();
        let announcement_count = channels
            .iter()
            .filter(|channel| channel.kind == ChannelType::GuildAnnouncement)
            .count();
        let stage_count = channels
            .iter()
            .filter(|channel| channel.kind == ChannelType::GuildStageVoice)
            .count();

        let mut features_vec = guild
            .features
            .into_iter()
            .map(|feature| feature.into())
            .collect::<Vec<Cow<'static, str>>>();
        features_vec.sort();
        let features = features_vec
            .iter()
            .map(|str| format!("\n- `{str}`"))
            .collect::<String>();
        let embed = EmbedBuilder::new()
            .color(0x41_A0_DE)
            .field(EmbedFieldBuilder::new(
                format!("<:community:1131779566000681062> {serverinfo_embed_generalinfo_field_name}"),
                format!(
                    "{} {}\n{} {}\n{} {}\n{} {}",
                    serverinfo_embed_generalinfo_id_subfield_name.to_string().discord_bold(),
                    guild.id.to_string().discord_inline_code(),
                    serverinfo_embed_generalinfo_created_subfield_name.to_string().discord_bold(),
                    (guild.id.timestamp() / 1000).to_string().discord_relative_timestamp(),
                    serverinfo_embed_generalinfo_owner_subfield_name.to_string().discord_bold(),
                    guild.owner_id.mention(),
                    serverinfo_embed_generalinfo_enabled_features_subfield_name.to_string().discord_bold(),
                    features,
                ),
            ))
            .field(EmbedFieldBuilder::new(
                format!("<:channels:1131857444809752576> {serverinfo_embed_channelinfo_field_name}"),
                format!(
                    "{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}",
                    "<:category:1131915276980600872>",
                    serverinfo_embed_channelinfo_categories_subfield_name.to_string().discord_bold(),
                    category_count,
                    "<:textChannel:1131860470488375316>",
                    serverinfo_embed_channelinfo_textchannels_subfield_name.to_string().discord_bold(),
                    text_count,
                    "<:voiceChannel:1131908258945318923>",
                    serverinfo_embed_channelinfo_voicechannels_subfield_name.to_string().discord_bold(),
                    voice_count,
                    "<:announcement:1131923904324186296>",
                    serverinfo_embed_channelinfo_announcementchannels_subfield_name.to_string().discord_bold(),
                    announcement_count,
                    "<:stage:1131926172574421032>",
                    serverinfo_embed_channelinfo_stagechannels_subfield_name.to_string().discord_bold(),
                    stage_count,
                ),
            ))
            .thumbnail(ImageSource::url(Cdn::guild_icon(guild.id, guild.icon.unwrap())).into_diagnostic()?)
            .title(guild.name)
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
