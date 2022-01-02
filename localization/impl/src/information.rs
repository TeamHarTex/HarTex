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

//! # `global` Module
//!
//! This module implements helper localization structures for the `Information` plugin.

use hartex_base::error::HarTexResult;
use hartex_conftoml::guildconf::locale::Locale as LocaleEnum;
use hartex_locale::Locale;

/// # Struct `GuildinfoCmdLocalize`
///
/// Localization helper structure for `guildinfo` command of the `information`
/// plugin.
pub struct GuildinfoCmdLocalize {
    pub embed_author: String,
    pub embed_guild_name_field: String,
    pub embed_guild_id_field: String,
    pub embed_guild_owner_field: String,
    pub embed_guild_owner_user_id_field: String,
    pub embed_guild_voice_regs_field: String,
    pub embed_guild_creation_date_field: String,
    pub embed_guild_members_field: String,
    pub embed_guild_members_fieldval_humans_part: String,
    pub embed_guild_members_fieldval_bots_part: String,
    pub embed_guild_channels_field: String,
    pub embed_guild_channels_fieldval_categories_part: String,
    pub embed_guild_channels_fieldval_texts_part: String,
    pub embed_guild_channels_fieldval_voices_part: String,
    pub embed_guild_channels_fieldval_stages_part: String,
    pub embed_guild_channels_fieldval_news_part: String,
    pub embed_guild_features_field: String,
    pub embed_guild_verify_lvl_field: String
}

impl GuildinfoCmdLocalize {
    pub fn init(locale: LocaleEnum) -> HarTexResult<Self> {
        let locale_file = Locale::load(&format!("../../_langcfgs/{locale}.langcfg"))?;

        Ok(Self {
            embed_author: locale_file["InformationPlugin.GuildinfoCommand.EmbedAuthor"].clone(),
            embed_guild_name_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildNameFieldName"].clone(),
            embed_guild_id_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildIdFieldName"].clone(),
            embed_guild_owner_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildOwnerFieldName"].clone(),
            embed_guild_owner_user_id_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildOwnerUserIdFieldName"]
                    .clone(),
            embed_guild_voice_regs_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildVoiceRegionsFieldName"]
                    .clone(),
            embed_guild_creation_date_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildCreationDatetimeFieldName"]
                    .clone(),
            embed_guild_members_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildMembersFieldName"]
                    .clone(),
            embed_guild_members_fieldval_humans_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildMembersFieldValueHumansPart"]
                    .clone(),
            embed_guild_members_fieldval_bots_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildMembersFieldValueBotsPart"]
                    .clone(),
            embed_guild_channels_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildChannelsFieldName"].clone(),
            embed_guild_channels_fieldval_categories_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildChannelsFieldValueCategoriesPart"]
                    .clone(),
            embed_guild_channels_fieldval_texts_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildChannelsFieldValueTextChannelsPart"]
                    .clone(),
            embed_guild_channels_fieldval_voices_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildChannelsFieldValueVoiceChannelsPart"]
                    .clone(),
            embed_guild_channels_fieldval_stages_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildChannelsFieldValueStageChannelsPart"]
                    .clone(),
            embed_guild_channels_fieldval_news_part:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildChannelsFieldValueNewsChannelsPart"]
                    .clone(),
            embed_guild_features_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildFeaturesFieldName"].clone(),
            embed_guild_verify_lvl_field:
                locale_file["InformationPlugin.GuildinfoCommand.EmbedGuildVerificationLevelFieldName"]
                    .clone()
        })
    }
}
