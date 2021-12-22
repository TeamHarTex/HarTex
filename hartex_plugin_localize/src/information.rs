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
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # `global` Module
//!
//! This module implements helper localization structures for the `Information` plugin.

use hartex_conftoml::guildconf::locale::Locale as LocaleEnum;
use hartex_core::error::HarTexResult;
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
