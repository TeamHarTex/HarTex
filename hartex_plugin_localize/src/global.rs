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
//! This module implements helper structures for the `Global` plugin.

use hartex_conftoml::guildconf::locale::Locale as LocaleEnum;
use hartex_locale::Locale;

/// # Struct `AboutCmdLocalize`
///
/// Localization helper structure for `about` command of the `global`
/// plugin.
pub struct AboutCmdLocalize {
    pub embed_desc: String,
    pub embed_botver_field: String,
    pub embed_whiteguilds_field: String
}

impl AboutCmdLocalize {
    /// # Static Method `AboutCmdLocalize::init`
    ///
    /// Initializes localization for the `about` command.
    pub fn init(locale: LocaleEnum) -> Self {
        let locale_file = Locale::load(&format!("../../langcfgs/{locale}.langcfg"))?;

        Self {
            embed_desc: locale_file["GlobalPlugin.AboutCommand.EmbedDescription"].clone(),
            embed_botver_field: locale_file["GlobalPlugin.AboutCommand.EmbedBotVersionFieldName"].clone(),
            embed_whiteguilds_field: locale_file["GlobalPlugin.AboutCommand.EmbedWhitelistedGuildsFieldName"].clone()
        }
    }
}
