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
//! This module implements helper localization structures for the `Global` plugin.

use hartex_base::error::HarTexResult;
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
    pub fn init(locale: LocaleEnum) -> HarTexResult<Self> {
        let locale_file = Locale::load(&format!("../../_langcfgs/{locale}.langcfg"))?;

        Ok(Self {
            embed_desc: locale_file["GlobalPlugin.AboutCommand.EmbedDescription"].clone(),
            embed_botver_field: locale_file["GlobalPlugin.AboutCommand.EmbedBotVersionFieldName"]
                .clone(),
            embed_whiteguilds_field:
                locale_file["GlobalPlugin.AboutCommand.EmbedWhitelistedGuildsFieldName"].clone()
        })
    }
}

/// # Struct `PingCmdLocalize`
///
/// Localization helper structure for `ping` command of the `global`
/// plugin.
pub struct PingCmdLocalize {
    pub init_resp: String,
    pub ms_unit: String
}

impl PingCmdLocalize {
    /// # Static Method `PingCmdLocalize::init`
    ///
    /// Initializes localization for the `ping` command.
    pub fn init(locale: LocaleEnum) -> HarTexResult<Self> {
        let locale_file = Locale::load(&format!("../../_langcfgs/{locale}.langcfg"))?;

        Ok(Self {
            init_resp: locale_file["GlobalPlugin.PingCommand.InitialResponse"].clone(),
            ms_unit: locale_file["GlobalPlugin.PingCommand.MillisecondUnit"].clone()
        })
    }
}

/// # Struct `SourceCmdLocalize`
///
/// Localization helper structure for `source` command of the `global`
/// plugin.
pub struct SourceCmdLocalize {
    pub prerepo_uri_msg: String
}

impl SourceCmdLocalize {
    /// # Static Method `SourceCmdLocalize::init`
    ///
    /// Initializes localization for the `source` command.
    pub fn init(locale: LocaleEnum) -> HarTexResult<Self> {
        let locale_file = Locale::load(&format!("../../_langcfgs/{locale}.langcfg"))?;

        Ok(Self {
            prerepo_uri_msg: locale_file["GlobalPlugin.SourceCommand.PreRepositoryUriMessage"]
                .clone()
        })
    }
}

/// # Struct `TeamCmdLocalize`
///
/// Localization helper structure for `team` command of the `global`
/// plugin.
pub struct TeamCmdLocalize {
    pub embed_title: String,
    pub embed_globadmin_leaddev_field: String,
    pub embed_contrib_field: String
}

impl TeamCmdLocalize {
    /// # Static Method `TeamCmdLocalize::init`
    ///
    /// Initializes localization for the `team` command.
    pub fn init(locale: LocaleEnum) -> HarTexResult<Self> {
        let locale_file = Locale::load(&format!("../../_langcfgs/{locale}.langcfg"))?;

        Ok(Self {
            embed_title: locale_file["GlobalPlugin.TeamCommand.EmbedTitle"].clone(),
            embed_globadmin_leaddev_field:
                locale_file["GlobalPlugin.TeamCommand.EmbedGlobalAdminAndLeadDevFieldName"].clone(),
            embed_contrib_field:
                locale_file["GlobalPlugin.TeamCommand.EmbedOtherContributorsFieldName"].clone()
        })
    }
}
