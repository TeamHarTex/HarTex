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

//! # Guild Localizable Objects

use hartex_discord_core::discord::model::guild::DefaultMessageNotificationLevel;
use hartex_discord_core::discord::model::guild::ExplicitContentFilter;
use hartex_discord_core::discord::model::guild::MfaLevel;
use hartex_discord_core::discord::model::guild::PremiumTier;
use hartex_discord_core::discord::model::guild::VerificationLevel;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use unic_langid::LanguageIdentifier;

use crate::localizable::Localizable;

impl Localizable for DefaultMessageNotificationLevel {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let locale = locale.map_or(String::from("en-GB"), |locale| locale.to_string());
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        Ok(match self {
            Self::All => localizer.guild_default_message_notification_level_all()?,
            Self::Mentions => localizer.guild_default_message_notification_level_mentions()?,
            _ => localizer.general_enum_unknown()?,
        })
    }
}

impl Localizable for ExplicitContentFilter {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let locale = locale.map_or(String::from("en-GB"), |locale| locale.to_string());
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        Ok(match self {
            Self::None => localizer.guild_explicit_content_filter_disabled()?,
            Self::MembersWithoutRole => localizer.guild_explicit_content_filter_members_without_roles()?,
            Self::AllMembers => localizer.guild_explicit_content_filter_all_members()?,
            _ => localizer.general_enum_unknown()?,
        })
    }
}

impl Localizable for MfaLevel {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let locale = locale.map_or(String::from("en-GB"), |locale| locale.to_string());
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        Ok(match self {
            Self::None => localizer.guild_mfa_level_none()?,
            Self::Elevated => localizer.guild_mfa_level_elevated()?,
            _ => localizer.general_enum_unknown()?,
        })
    }
}

impl Localizable for PremiumTier {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let locale = locale.map_or(String::from("en-GB"), |locale| locale.to_string());
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        Ok(match self {
            Self::None => localizer.guild_premium_tier_none()?,
            Self::Tier1 => localizer.guild_premium_tier_one()?,
            Self::Tier2 => localizer.guild_premium_tier_two()?,
            Self::Tier3 => localizer.guild_premium_tier_three()?,
            _ => localizer.general_enum_unknown()?,
        })
    }
}

impl Localizable for VerificationLevel {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let locale = locale.map_or(String::from("en-GB"), |locale| locale.to_string());
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        Ok(match self {
            Self::None => localizer.guild_verification_level_none()?,
            Self::Low => localizer.guild_verification_level_low()?,
            Self::Medium => localizer.guild_verification_level_medium()?,
            Self::High => localizer.guild_verification_level_high()?,
            Self::VeryHigh => localizer.guild_verification_level_very_high()?,
            _ => localizer.general_enum_unknown()?,
        })
    }
}
