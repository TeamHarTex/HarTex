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

//! # Guild Entities

use hartex_discord_entitycache_core::entity;

/// A guild entity.
#[allow(clippy::module_name_repetitions)]
#[entity(
    from = "twilight_model::guild::Guild",
    assume = ["CachedGuildSelectById"],
    id = ["id"],
    include = [
        "default_message_notifications",
        "explicit_content_filter",
        "features",
        "icon",
        "large",
        "mfa_level",
        "name",
        "premium_subscription_count",
        "premium_tier",
        "owner_id",
        "verification_level"
    ],
    extra = [],
    overrides = [
        "DefaultMessageNotificationLevel": "twilight_model::guild::DefaultMessageNotificationLevel",
        "ExplicitContentFilter": "twilight_model::guild::ExplicitContentFilter",
        "GuildFeature": "twilight_model::guild::GuildFeature",
        "MfaLevel": "twilight_model::guild::MfaLevel",
        "PremiumTier": "twilight_model::guild::PremiumTier",
        "VerificationLevel": "twilight_model::guild::VerificationLevel"
    ],
    relates = [
        multiple "MemberEntity": via "id" as "guild_id",
    ],
)]
pub struct GuildEntity;
