/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! The `guildinfo` command.

use hartex_base::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        embed_builder::{
            EmbedAuthorBuilder,
            EmbedBuilder,
            EmbedFieldBuilder,
            ImageSource
        },
        model::{
            application::{
                callback::{
                    CallbackData,
                    InteractionResponse
                },
                interaction::Interaction
            },
            channel::ChannelType,
            guild::VerificationLevel
        },
        util::snowflake::Snowflake
    },
    error::{
        HarTexError,
        HarTexResult
    },
    is_stable,
    logging::tracing,
    time::{
        FixedOffset,
        TimeZone
    }
};
use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};
use hartex_conftoml::guildconf::{
    locale::Locale,
    tz::Timezone
};
use hartex_dbmani::guildconf::GetGuildConfig;
use hartex_localization_impl::information::GuildinfoCmdLocalize;
use hartex_utils::{
    cdn::{
        Cdn,
        CdnResourceFormat
    },
    FutureRetType
};

/// The `guildinfo` command.
pub struct Guildinfo;

impl Command for Guildinfo {
    fn name(&self) -> String {
        String::from("guildinfo")
    }

    fn description(&self) -> String {
        String::from("InformationPlugin.GuildinfoCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        cache: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_guildinfo_command(ctx, cache))
    }
}

/// Executes the `guildinfo` command.
#[allow(clippy::too_many_lines)]
async fn execute_guildinfo_command(
    ctx: CommandContext,
    cache: CloneableInMemoryCache
) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
        });
    };

    tracing::trace!("checking interaction source");

    if interaction.guild_id.is_none() || interaction.user.is_some() {
        tracing::error!("interaction source is not a guild, responding with such error");

        if let Err(error) = ctx
            .http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(String::from(
                        ":x: This command can only be used in a guild."
                    )),
                    embeds: None,
                    flags: None,
                    tts: None
                })
            )
            .exec()
            .await
        {
            tracing::error!("failed to respond to interaction: {error}");

            return Err(HarTexError::from(error));
        }
    }

    tracing::trace!("attempting to obtain guild config");

    // unwrapping here is fine as it is now ensured that the interaction is sent from a guild,
    // not in a user DM (which is the case when interaction.guild_id is None)

    let config = GetGuildConfig::new(interaction.guild_id.unwrap()).await?;

    let localize = if interaction.guild_id.is_none() || interaction.user.is_some() {
        GuildinfoCmdLocalize::init(Locale::EnGb)
            .expect("failed to load localization for guildinfo command")
    }
    else if !is_stable() && config.NightlyFeatures.localization {
        GuildinfoCmdLocalize::init(config.GuildConfiguration.locale)
            .expect("failed to load localization of guildinfo command")
    }
    else {
        GuildinfoCmdLocalize::init(Locale::EnGb)
            .expect("failed to load localization for guildinfo command")
    };

    tracing::trace!("attempting to obtain cached guild");

    let guild = cache.guild(interaction.guild_id.unwrap()).unwrap();

    tracing::trace!("attempting to obtain guild owner");

    let guild_owner = match {
        match ctx.http.user(guild.owner_id()).exec().await {
            Ok(response) => response,
            Err(error) => {
                tracing::error!("failed to receive request response: {error}");

                return Err(HarTexError::from(error));
            }
        }
        .model()
        .await
    } {
        Ok(user) => user,
        Err(error) => {
            tracing::error!("failed to deserialize response body: {error}");

            return Err(HarTexError::from(error));
        }
    };

    tracing::trace!("attempting to obtain guild member list");

    // it is ok to call unwrap here because we are sure that the limit never exceeds 1000
    let guild_members = match {
        match ctx
            .http
            .guild_members(guild.id())
            .limit(1000)
            .unwrap()
            .exec()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                tracing::error!("failed to receive request response: {error}");

                return Err(HarTexError::from(error));
            }
        }
        .models()
        .await
    } {
        Ok(members) => members,
        Err(error) => {
            tracing::error!("failed to deserialize response body: {error}");

            return Err(HarTexError::from(error));
        }
    };

    tracing::trace!("attempting to obtain guild channel list");

    let guild_channels = match {
        match ctx.http.guild_channels(guild.id()).exec().await {
            Ok(response) => response,
            Err(error) => {
                tracing::error!("failed to receive request response: {error}");

                return Err(HarTexError::from(error));
            }
        }
        .models()
        .await
    } {
        Ok(channels) => channels,
        Err(error) => {
            tracing::error!("failed to deserialize response body: {error}");

            return Err(HarTexError::from(error));
        }
    };

    tracing::trace!("attempting to obtain guild voice region list");

    let guild_voice_regions = match {
        match ctx.http.guild_voice_regions(guild.id()).exec().await {
            Ok(response) => response,
            Err(error) => {
                tracing::error!("failed to receive request response: {error}");

                return Err(HarTexError::from(error));
            }
        }
        .models()
        .await
    } {
        Ok(regions) => regions,
        Err(error) => {
            tracing::error!("failed to deserialize response body: {error}");

            return Err(HarTexError::from(error));
        }
    };

    let guild_member_count = guild_members.len();

    let guild_user_count = guild_members
        .iter()
        .filter(|member| !member.user.bot)
        .count();

    let channels_iter = guild_channels.iter();

    let categories = channels_iter
        .clone()
        .filter(|channel| channel.kind() == ChannelType::GuildCategory)
        .count();
    let texts = channels_iter
        .clone()
        .filter(|channel| channel.kind() == ChannelType::GuildText)
        .count();
    let voices = channels_iter
        .clone()
        .filter(|channel| channel.kind() == ChannelType::GuildVoice)
        .count();
    let stages = channels_iter
        .clone()
        .filter(|channel| channel.kind() == ChannelType::GuildStageVoice)
        .count();
    let news = channels_iter
        .filter(|channel| channel.kind() == ChannelType::GuildNews)
        .count();

    let icon_url = if let Some(hash) = guild.icon() {
        let format = if hash.starts_with("a_") {
            CdnResourceFormat::GIF
        }
        else {
            CdnResourceFormat::PNG
        };

        Cdn::guild_icon(guild.id(), hash, &format)
    }
    else {
        String::new()
    };

    let mut author = EmbedAuthorBuilder::new(format!(
        "{info_about} {name}",
        info_about = localize.embed_author,
        name = &guild.name()
    ));

    if !icon_url.is_empty() {
        let temp = author.clone();

        author = temp.icon_url(ImageSource::url(icon_url)?);
    }

    let voice_regions_repr_str = guild_voice_regions
        .iter()
        .map(|region| format!("`{region}`", region = &region.name))
        .collect::<Vec<_>>();

    let mut embed = EmbedBuilder::new()
        .author(author)
        .color(0x0003_BEFC)
        .field(EmbedFieldBuilder::new(localize.embed_guild_name_field, guild.name()).inline())
        .field(
            EmbedFieldBuilder::new(
                localize.embed_guild_id_field,
                format!("{id}", id = guild.id())
            )
            .inline()
        )
        .field(EmbedFieldBuilder::new(
            localize.embed_guild_owner_field,
            format!(
                "{name}#{discriminator}",
                name = guild_owner.name,
                discriminator = guild_owner.discriminator
            )
        ))
        .field(EmbedFieldBuilder::new(
            localize.embed_guild_owner_user_id_field,
            format!("{id}", id = guild_owner.id)
        ))
        .field(EmbedFieldBuilder::new(
            localize.embed_guild_voice_regs_field,
            voice_regions_repr_str.join(", ")
        ));

    let timezone = if config.NightlyFeatures.localization && !is_stable() {
        config.GuildConfiguration.timezone
    }
    else {
        Timezone::UTC
    };

    let created_at =
        FixedOffset::east(timezone.into_offset_secs()).timestamp_millis(guild.id().timestamp());

    let features = guild.features();
    let features_vec = features
        .map(|feature| format!("`{feature}`"))
        .collect::<Vec<_>>();

    let mut features_str = features_vec.join("\n - ");

    if features_str.is_empty() {
        features_str = String::from("none");
    }

    let verification_level = match guild.verification_level() {
        VerificationLevel::None => "none",
        VerificationLevel::Low => "low",
        VerificationLevel::Medium => "medium",
        VerificationLevel::High => "high",
        VerificationLevel::VeryHigh => "very high"
    };

    let temp = embed.clone();

    embed = temp
        .field(EmbedFieldBuilder::new(localize.embed_guild_creation_date_field, format!("{created_at} ({timezone})")).inline())
        .field(EmbedFieldBuilder::new(
            format!(
                "{members} - {guild_member_count}",
                members = localize.embed_guild_members_field,
            ),
            format!(
                "{humans}: {guild_user_count}\n{bots_text}: {bots}",
                humans = localize.embed_guild_members_fieldval_humans_part,
                bots_text = localize.embed_guild_members_fieldval_bots_part,
                bots = guild_member_count as usize - guild_user_count
            )
        ))
        .field(EmbedFieldBuilder::new(
            format!("{channels} - {total}",
                channels = localize.embed_guild_channels_field,
                total = guild_channels.len()
            ),
            format!(
                "{categories_text}: {categories}\n{texts_text}: {texts}\n{voices_text}: {voices}\n{stages_text}: {stages}\n{news_text}: {news}",
                categories_text = localize.embed_guild_channels_fieldval_categories_part,
                texts_text = localize.embed_guild_channels_fieldval_texts_part,
                voices_text = localize.embed_guild_channels_fieldval_voices_part,
                stages_text = localize.embed_guild_channels_fieldval_stages_part,
                news_text = localize.embed_guild_channels_fieldval_news_part
            )
        ))
        .field(EmbedFieldBuilder::new(localize.embed_guild_features_field, format!("- {features_str}")))
        .field(EmbedFieldBuilder::new(localize.embed_guild_verify_lvl_field, verification_level));

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: None,
                embeds: Some(vec![embed.build()?]),
                flags: None,
                tts: None
            })
        )
        .exec()
        .await?;

    Ok(())
}
