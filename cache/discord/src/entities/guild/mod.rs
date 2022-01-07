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

//! # The `guild` Module
//!
//! This module contains entities related to Discord guilds.

use std::slice::Iter;

use hartex_base::{
    discord::model::{
        datetime::Timestamp,
        guild::{
            DefaultMessageNotificationLevel,
            ExplicitContentFilter,
            Guild,
            MfaLevel,
            NSFWLevel,
            Permissions,
            PremiumTier,
            SystemChannelFlags,
            VerificationLevel
        },
        id::{
            ApplicationId,
            ChannelId,
            GuildId,
            UserId
        }
    },
    stdext::prelude::*
};
use hartex_cache_base::entity::Entity;

pub mod emoji;
pub mod member;
pub mod role;

/// # Struct `GuildEntity`
///
/// A guild entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct GuildEntity {
    afk_channel_id: Option<ChannelId>,
    afk_timeout: u64,
    application_id: Option<ApplicationId>,
    approximate_member_count: Option<u64>,
    approximate_presence_count: Option<u64>,
    banner: Option<String>,
    default_message_notifications: DefaultMessageNotificationLevel,
    description: Option<String>,
    discovery_splash: Option<String>,
    explicit_content_filter: ExplicitContentFilter,
    features: Vec<String>,
    icon: Option<String>,
    id: GuildId,
    joined_at: Option<Timestamp>,
    large: bool,
    max_members: Option<u64>,
    max_presence: Option<u64>,
    max_video_channel_users: Option<u64>,
    member_count: Option<u64>,
    mfa_level: MfaLevel,
    name: String,
    nsfw_level: NSFWLevel,
    owner_id: UserId,
    owner: Option<bool>,
    permissions: Option<Permissions>,
    preferred_locale: String,
    premium_subscription_count: Option<u64>,
    premium_tier: PremiumTier,
    rules_channel_id: Option<ChannelId>,
    splash: Option<String>,
    system_channel_flags: SystemChannelFlags,
    system_channel_id: Option<ChannelId>,
    unavailable: bool,
    vanity_url_code: Option<String>,
    verification_level: VerificationLevel,
    widget_channel_id: Option<ChannelId>,
    widget_enabled: Option<bool>
}

impl GuildEntity {
    #[must_use]
    pub fn afk_channel_id(&self) -> Option<ChannelId> {
        self.afk_channel_id
    }

    #[must_use]
    pub fn afk_timeout(&self) -> u64 {
        self.afk_timeout
    }

    #[must_use]
    pub fn application_id(&self) -> Option<ApplicationId> {
        self.application_id
    }

    #[must_use]
    pub fn approximate_member_count(&self) -> Option<u64> {
        self.approximate_member_count
    }

    #[must_use]
    pub fn approximate_presence_count(&self) -> Option<u64> {
        self.approximate_presence_count
    }

    #[must_use]
    pub fn banner(&self) -> Option<&str> {
        self.banner.as_refstr()
    }

    #[must_use]
    pub fn default_message_notifications(&self) -> DefaultMessageNotificationLevel {
        self.default_message_notifications
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_refstr()
    }

    #[must_use]
    pub fn discovery_splash(&self) -> Option<&str> {
        self.discovery_splash.as_refstr()
    }

    #[must_use]
    pub fn explicit_content_filter(&self) -> ExplicitContentFilter {
        self.explicit_content_filter
    }

    #[must_use]
    pub fn features(&self) -> GuildFeaturesIter<'_> {
        GuildFeaturesIter(self.features.iter())
    }

    #[must_use]
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_refstr()
    }

    #[must_use]
    pub fn joined_at(&self) -> Option<Timestamp> {
        self.joined_at
    }

    #[must_use]
    pub fn large(&self) -> bool {
        self.large
    }

    #[must_use]
    pub fn max_members(&self) -> Option<u64> {
        self.max_members
    }

    #[must_use]
    pub fn max_presence(&self) -> Option<u64> {
        self.max_presence
    }

    #[must_use]
    pub fn max_video_channel_users(&self) -> Option<u64> {
        self.max_video_channel_users
    }

    #[must_use]
    pub fn member_count(&self) -> Option<u64> {
        self.member_count
    }

    #[must_use]
    pub fn mfa_level(&self) -> MfaLevel {
        self.mfa_level
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn nsfw_level(&self) -> NSFWLevel {
        self.nsfw_level
    }

    #[must_use]
    pub fn owner_id(&self) -> UserId {
        self.owner_id
    }

    #[must_use]
    pub fn owner(&self) -> Option<bool> {
        self.owner
    }

    #[must_use]
    pub fn permissions(&self) -> Option<Permissions> {
        self.permissions
    }

    #[must_use]
    pub fn preferred_locale(&self) -> &str {
        self.preferred_locale.as_ref()
    }

    #[must_use]
    pub fn premium_subscription_count(&self) -> Option<u64> {
        self.premium_subscription_count
    }

    #[must_use]
    pub fn premium_tier(&self) -> PremiumTier {
        self.premium_tier
    }

    #[must_use]
    pub fn rules_channel_id(&self) -> Option<ChannelId> {
        self.rules_channel_id
    }

    #[must_use]
    pub fn splash(&self) -> Option<&str> {
        self.splash.as_refstr()
    }

    #[must_use]
    pub fn system_channel_flags(&self) -> SystemChannelFlags {
        self.system_channel_flags
    }

    #[must_use]
    pub fn system_channel_id(&self) -> Option<ChannelId> {
        self.system_channel_id
    }

    #[must_use]
    pub fn unavailable(&self) -> bool {
        self.unavailable
    }

    #[must_use]
    pub fn vanity_url_code(&self) -> Option<&str> {
        self.vanity_url_code.as_refstr()
    }

    #[must_use]
    pub fn verification_level(&self) -> VerificationLevel {
        self.verification_level
    }

    #[must_use]
    pub fn widget_channel_id(&self) -> Option<ChannelId> {
        self.widget_channel_id
    }

    #[must_use]
    pub fn widget_enabled(&self) -> Option<bool> {
        self.widget_enabled
    }
}

impl Entity for GuildEntity {
    type Id = GuildId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<Guild> for GuildEntity {
    fn from(guild: Guild) -> Self {
        Self {
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            approximate_member_count: guild.approximate_member_count,
            approximate_presence_count: guild.approximate_presence_count,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features,
            icon: guild.icon,
            id: guild.id,
            joined_at: guild.joined_at,
            large: guild.large,
            max_members: guild.max_members,
            max_presence: guild.max_presences,
            max_video_channel_users: guild.max_video_channel_users,
            member_count: guild.member_count,
            mfa_level: guild.mfa_level,
            name: guild.name,
            nsfw_level: guild.nsfw_level,
            owner_id: guild.owner_id,
            owner: guild.owner,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            rules_channel_id: guild.rules_channel_id,
            splash: guild.splash,
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: guild.system_channel_id,
            unavailable: guild.unavailable,
            vanity_url_code: guild.vanity_url_code,
            verification_level: guild.verification_level,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct GuildFeaturesIter<'a>(Iter<'a, String>);

impl<'a> Iterator for GuildFeaturesIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(AsRef::as_ref)
    }
}
