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

//! # The Info Role Subcommand
//!
//! This command returns informatiomn about the current server (the server the command is run in).

use std::{borrow::Cow, fmt::Write};

use hartex_discord_cdn::Cdn;
use hartex_discord_commands_core::context::CommandContext;
use hartex_discord_core::discord::{
    mention::Mention,
    model::{
        application::interaction::application_command::CommandDataOption, channel::ChannelType,
    },
    util::{
        builder::embed::{EmbedBuilder, EmbedFieldBuilder, ImageSource},
        snowflake::Snowflake,
    },
};
use hartex_discord_utils::{
    commands::{CommandDataOptionExt, CommandDataOptionsExt},
    interaction::embed_response,
    localizable::Localizable,
    markdown::MarkdownStyle,
};
use miette::IntoDiagnostic;

/// Executes the `info server` command.
#[allow(clippy::too_many_lines)]
pub async fn execute(
    context: &CommandContext<'_>,
    option: &CommandDataOption,
) -> miette::Result<()> {
    let options = option.assume_subcommand();

    let langid_locale = context
        .locale
        .as_ref()
        .and_then(|locale| locale.parse().ok());

    let verbose = options.boolean_value_of("verbose");

    let Some(guild) = context.cache.guild(context.guild.unwrap()) else {
        unreachable!()
    };

    let serverinfo_embed_generalinfo_id_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_generalinfo_id_subfield_name()?;
    let serverinfo_embed_generalinfo_created_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_generalinfo_created_subfield_name()?;
    let serverinfo_embed_generalinfo_owner_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_generalinfo_owner_subfield_name()?;
    let serverinfo_embed_generalinfo_enabled_features_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_generalinfo_enabled_features_subfield_name()?;
    let serverinfo_embed_generalinfo_field_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_generalinfo_field_name()?;
    let serverinfo_embed_channelinfo_field_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_field_name()?;
    let serverinfo_embed_channelinfo_categories_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_categories_subfield_name()?;
    let serverinfo_embed_channelinfo_textchannels_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_textchannels_subfield_name()?;
    let serverinfo_embed_channelinfo_voicechannels_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_voicechannels_subfield_name()?;
    let serverinfo_embed_channelinfo_announcementchannels_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_announcementchannels_subfield_name()?;
    let serverinfo_embed_channelinfo_stagechannels_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_stagechannels_subfield_name()?;
    let serverinfo_embed_channelinfo_forumchannels_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_channelinfo_forumchannels_subfield_name()?;
    let serverinfo_embed_memberinfo_field_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_memberinfo_field_name()?;
    let serverinfo_embed_memberinfo_membercount_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_memberinfo_membercount_subfield_name()?;
    let serverinfo_embed_memberinfo_humancount_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_memberinfo_humancount_subfield_name()?;
    let serverinfo_embed_memberinfo_botcount_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_memberinfo_botcount_subfield_name()?;
    let serverinfo_embed_roleinfo_field_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_roleinfo_field_name()?;
    let serverinfo_embed_roleinfo_rolecount_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_roleinfo_rolecount_subfield_name()?;
    let serverinfo_embed_nitroinfo_field_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_nitroinfo_field_name()?;
    let serverinfo_embed_nitroinfo_boostlevel_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_nitroinfo_field_boostlevel_subfield_name()?;
    let serverinfo_embed_nitroinfo_boosts_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_nitroinfo_field_boosts_subfield_name()?;
    let serverinfo_embed_flags_field_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_flags_field_name()?;
    let serverinfo_embed_flags_large_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_flags_large_subfield_name()?;
    let serverinfo_embed_flags_default_message_notifications_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_flags_default_message_notifications_subfield_name()?;
    let serverinfo_embed_flags_explicit_content_filter_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_flags_explicit_content_filter_subfield_name()?;
    let serverinfo_embed_flags_mfa_level_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_flags_mfa_level_subfield_name()?;
    let serverinfo_embed_flags_verification_level_subfield_name = context
        .localizer
        .utilities_plugin_serverinfo_embed_flags_verification_level_subfield_name()?;

    let mut default_general_information = format!(
        "{} {}\n{} {}\n{} {}",
        serverinfo_embed_generalinfo_id_subfield_name,
        guild.id().to_string().discord_inline_code(),
        serverinfo_embed_generalinfo_created_subfield_name,
        (guild.id().timestamp() / 1000)
            .to_string()
            .discord_relative_timestamp(),
        serverinfo_embed_generalinfo_owner_subfield_name,
        guild.owner_id().mention(),
    );
    let Some(channel_ids) = context.cache.guild_channels(guild.id()) else {
        unreachable!()
    };
    let channels = channel_ids
        .iter()
        .filter_map(|id| context.cache.channel(*id));

    let category_count = channels
        .clone()
        .filter(|channel| channel.kind == ChannelType::GuildCategory)
        .count();
    let text_count = channels
        .clone()
        .filter(|channel| channel.kind == ChannelType::GuildText)
        .count();
    let voice_count = channels
        .clone()
        .filter(|channel| channel.kind == ChannelType::GuildVoice)
        .count();
    let announcement_count = channels
        .clone()
        .filter(|channel| channel.kind == ChannelType::GuildAnnouncement)
        .count();
    let stage_count = channels
        .clone()
        .filter(|channel| channel.kind == ChannelType::GuildStageVoice)
        .count();
    let forum_count = channels
        .filter(|channel| channel.kind == ChannelType::GuildForum)
        .count();

    let features = guild
        .features()
        .cloned()
        .map::<Cow<'static, str>, _>(Into::into)
        .fold(String::new(), |mut output, feature| {
            let _ = write!(output, "\n- `{feature}`");
            output
        });

    let Some(member_ids) = context.cache.guild_members(guild.id()) else {
        unreachable!()
    };
    let users = member_ids.iter().filter_map(|id| context.cache.user(*id));
    let humans = users.filter(|user| !user.bot).count();

    if verbose {
        write!(
            default_general_information,
            "\n {serverinfo_embed_generalinfo_enabled_features_subfield_name} {features}"
        )
        .into_diagnostic()?;
    }

    let Some(role_ids) = context.cache.guild_roles(guild.id()) else {
        unreachable!()
    };
    let roles = role_ids.len();

    let mut builder = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            format!("<:community:1190564037428252763> {serverinfo_embed_generalinfo_field_name}"),
            default_general_information,
        ))
        .field(EmbedFieldBuilder::new(
            format!("<:channels:1131857444809752576> {serverinfo_embed_channelinfo_field_name}"),
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
        ))
        .field(EmbedFieldBuilder::new(
            format!("<:members:1132582503157334016> {serverinfo_embed_memberinfo_field_name}"),
            format!(
                "{} {}\n{} {}\n{} {}",
                serverinfo_embed_memberinfo_membercount_subfield_name,
                member_ids.len(),
                serverinfo_embed_memberinfo_humancount_subfield_name,
                humans,
                serverinfo_embed_memberinfo_botcount_subfield_name,
                member_ids.len() - humans,
            ),
        ))
        .field(EmbedFieldBuilder::new(
            format!("<:role:1139004530277765211> {serverinfo_embed_roleinfo_field_name}"),
            format!("{serverinfo_embed_roleinfo_rolecount_subfield_name} {roles}",),
        ))
        .field(EmbedFieldBuilder::new(
            format!("<:nitroBoost:1190566150963200030> {serverinfo_embed_nitroinfo_field_name}"),
            format!(
                "{} {}\n{} {}",
                serverinfo_embed_nitroinfo_boostlevel_subfield_name,
                guild.premium_tier().localize(langid_locale.clone())?,
                serverinfo_embed_nitroinfo_boosts_subfield_name,
                guild.premium_subscription_count().unwrap_or_default(),
            ),
        ))
        .field(EmbedFieldBuilder::new(
            serverinfo_embed_flags_field_name,
            format!(
                "{} {}\n{} {}\n{} {}\n{} {}\n{} {}",
                serverinfo_embed_flags_large_subfield_name,
                guild.large().localize(langid_locale.clone())?,
                serverinfo_embed_flags_default_message_notifications_subfield_name,
                guild
                    .default_message_notifications()
                    .localize(langid_locale.clone())?,
                serverinfo_embed_flags_explicit_content_filter_subfield_name,
                guild
                    .explicit_content_filter()
                    .localize(langid_locale.clone())?,
                serverinfo_embed_flags_mfa_level_subfield_name,
                guild.mfa_level().localize(langid_locale.clone())?,
                serverinfo_embed_flags_verification_level_subfield_name,
                guild.verification_level().localize(langid_locale)?,
            ),
        ))
        .title(guild.name());

    if let Some(icon) = guild.icon() {
        builder = builder
            .thumbnail(ImageSource::url(Cdn::guild_icon(guild.id(), *icon)).into_diagnostic()?);
    }

    let embed = builder.validate().into_diagnostic()?.build();

    context.create_response(embed_response(vec![embed])).await?;

    Ok(())
}
