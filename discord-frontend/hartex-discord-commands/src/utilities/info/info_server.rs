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
use std::fmt::Write;

use futures::future;
use hartex_discord_cdn::Cdn;
use hartex_discord_core::discord::mention::Mention;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
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
use hartex_discord_entitycache_repositories::member::CachedMemberRepository;
use hartex_discord_entitycache_repositories::role::CachedRoleRepository;
use hartex_discord_entitycache_repositories::user::CachedUserRepository;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;
use miette::IntoDiagnostic;

#[allow(clippy::too_many_lines)]
pub async fn execute(interaction: Interaction, option: CommandDataOption) -> miette::Result<()> {
    let CommandOptionValue::SubCommand(options) = option.value else {
        unreachable!()
    };

    let interaction_client = CLIENT.interaction(interaction.application_id);
    let locale = interaction.locale.and_then(|locale| locale.parse().ok());

    let bundle = create_bundle(
        locale.clone(),
        &["discord-frontend", "commands"],
    )?;

    let CommandOptionValue::Boolean(verbose) = options
        .iter()
        .find(|option| option.name.as_str() == "verbose")
        .map_or(CommandOptionValue::Boolean(false), |option| {
            option.value.clone()
        })
    else {
        unreachable!()
    };

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
    bundle_get!(bundle."serverinfo-embed-channelinfo-forumchannels-subfield-name": message, out [serverinfo_embed_channelinfo_forumchannels_subfield_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-memberinfo-field-name": message, out [serverinfo_embed_memberinfo_field_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-memberinfo-membercount-subfield-name": message, out [serverinfo_embed_memberinfo_membercount_subfield_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-memberinfo-humancount-subfield-name": message, out [serverinfo_embed_memberinfo_humancount_subfield_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-memberinfo-botcount-subfield-name": message, out [serverinfo_embed_memberinfo_botcount_subfield_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-roleinfo-field-name": message, out [serverinfo_embed_roleinfo_field_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-roleinfo-rolecount-subfield-name": message, out [serverinfo_embed_roleinfo_rolecount_subfield_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-flags-field-name": message, out [serverinfo_embed_flags_field_name, errors]);
    handle_errors(errors)?;
    bundle_get!(bundle."serverinfo-embed-flags-large-subfield-name": message, out [serverinfo_embed_flags_large_subfield_name, errors]);
    handle_errors(errors)?;

    let mut default_general_information = format!(
        "{} {}\n{} {}\n{} {}",
        serverinfo_embed_generalinfo_id_subfield_name,
        guild.id.to_string().discord_inline_code(),
        serverinfo_embed_generalinfo_created_subfield_name,
        (guild.id.timestamp() / 1000)
            .to_string()
            .discord_relative_timestamp(),
        serverinfo_embed_generalinfo_owner_subfield_name,
        guild.owner_id.mention(),
    );

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
    let forum_count = channels
        .iter()
        .filter(|channel| channel.kind == ChannelType::GuildForum)
        .count();

    let mut features_vec = guild
        .features
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Cow<'static, str>>>();
    features_vec.sort();
    let features = features_vec
        .iter()
        .fold(String::new(), |mut output, feature| {
            let _ = write!(output, "\n- `{feature}`");
            output
        });

    let members = CachedMemberRepository
        .member_ids_in_guild(guild.id)
        .await
        .into_diagnostic()?;
    let users = future::try_join_all(members.iter().map(|id| CachedUserRepository.get(*id)))
        .await
        .into_diagnostic()?;
    let humans = users.iter().filter(|user| !user.bot).count();

    if verbose {
        default_general_information.push_str(&format!(
            "\n {serverinfo_embed_generalinfo_enabled_features_subfield_name} {features}",
        ));
    }

    let roles = CachedRoleRepository
        .role_ids_in_guild(guild.id)
        .into_diagnostic()?;

    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            format!("<:community:1131779566000681062> {serverinfo_embed_generalinfo_field_name}"),
            default_general_information,
        ))
        .field(
            EmbedFieldBuilder::new(
                format!(
                    "<:channels:1131857444809752576> {serverinfo_embed_channelinfo_field_name}"
                ),
                format!(
                    "{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}",
                    "<:category:1131915276980600872>",
                    serverinfo_embed_channelinfo_categories_subfield_name,
                    category_count,
                    "<:textChannel:1131860470488375316>",
                    serverinfo_embed_channelinfo_textchannels_subfield_name,
                    text_count,
                    "<:voiceChannel:1131908258945318923>",
                    serverinfo_embed_channelinfo_voicechannels_subfield_name,
                    voice_count,
                    "<:announcement:1131923904324186296>",
                    serverinfo_embed_channelinfo_announcementchannels_subfield_name,
                    announcement_count,
                    "<:stage:1131926172574421032>",
                    serverinfo_embed_channelinfo_stagechannels_subfield_name,
                    stage_count,
                    "<:forum:1131928666176241735>",
                    serverinfo_embed_channelinfo_forumchannels_subfield_name,
                    forum_count,
                ),
            )
            .inline(),
        )
        .field(
            EmbedFieldBuilder::new(
                format!("<:members:1132582503157334016> {serverinfo_embed_memberinfo_field_name}"),
                format!(
                    "{} {}\n{} {}\n{} {}",
                    serverinfo_embed_memberinfo_membercount_subfield_name,
                    members.len(),
                    serverinfo_embed_memberinfo_humancount_subfield_name,
                    humans,
                    serverinfo_embed_memberinfo_botcount_subfield_name,
                    members.len() - humans,
                ),
            )
            .inline(),
        )
        .field(EmbedFieldBuilder::new(
            format!("<:role:1139004530277765211> {serverinfo_embed_roleinfo_field_name}"),
            format!(
                "{} {}",
                serverinfo_embed_roleinfo_rolecount_subfield_name,
                roles.len(),
            ),
        ))
        .field(EmbedFieldBuilder::new(
            serverinfo_embed_flags_field_name,
            format!(
                "{} {}",
                serverinfo_embed_flags_large_subfield_name,
                if guild.large {
                    boolean_true
                } else {
                    boolean_false
                }
            )
        ))
        .thumbnail(
            ImageSource::url(Cdn::guild_icon(guild.id, guild.icon.unwrap())).into_diagnostic()?,
        )
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
