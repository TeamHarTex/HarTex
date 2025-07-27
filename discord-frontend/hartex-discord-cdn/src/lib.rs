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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_discord_core::discord::model::{
    id::{
        Id,
        marker::{GuildMarker, RoleMarker, UserMarker},
    },
    util::ImageHash,
};

/// Utility struct providing utility functions for working with the Discord content delivery
/// network.
pub struct Cdn;

impl Cdn {
    /// The base URL for all URLs linking to media served on the Discord content delivery
    /// network.
    pub const URL_BASE: &'static str = "https://cdn.discordapp.com/";

    /// Constructs a content delivery network URL for obtaining the default user avatar of a user.
    #[must_use]
    pub fn default_user_avatar(
        user_id: Option<Id<UserMarker>>,
        discriminator: Option<u16>,
    ) -> String {
        let index = if let Some(id) = user_id {
            (id.get() >> 22) % 6
        } else if let Some(discrim) = discriminator {
            u64::from(discrim % 5)
        } else {
            unreachable!()
        };

        formati::format!("{Self::URL_BASE}embed/avatars/{index}.png")
    }

    /// Constructs a content delivery network URL for obtaining the guild icon of a guild.
    #[must_use]
    pub fn guild_icon(guild_id: Id<GuildMarker>, icon: ImageHash) -> String {
        let mut url = formati::format!("{Self::URL_BASE}icons/{guild_id}/{icon}");
        if icon.is_animated() {
            url.push_str(".gif");
        } else {
            url.push_str(".png");
        }

        url
    }

    /// Constructs a content delivery network URL for obtaining the role icon of a role.
    #[must_use]
    pub fn role_icon(role_id: Id<RoleMarker>, icon: ImageHash) -> String {
        let mut url = formati::format!("{Self::URL_BASE}icons/{role_id}/{icon}");
        if icon.is_animated() {
            url.push_str(".gif");
        } else {
            url.push_str(".png");
        }

        url
    }

    /// Constructs a content delivery network URL for obtaining the user avatar of a user.
    #[must_use]
    pub fn user_avatar(user_id: Id<UserMarker>, avatar: ImageHash) -> String {
        let mut url = formati::format!("{Self::URL_BASE}avatars/{user_id}/{avatar}");
        if avatar.is_animated() {
            url.push_str(".gif");
        } else {
            url.push_str(".png");
        }

        url
    }
}
