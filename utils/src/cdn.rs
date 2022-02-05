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

//! A wrapper of the Discord CDN.

use std::fmt::{self, Display, Formatter};

use hartex_base::discord::model::id::{GuildId, UserId};

/// The "namespace" for various CDN endpoints
pub struct Cdn;

impl Cdn {
    #[must_use]
    pub fn default_user_avatar(discriminator: u16) -> String {
        format!(
            "https://cdn.discordapp.com/embed/avatars/{discriminator}.png",
            discriminator = discriminator % 5
        )
    }

    #[must_use]
    pub fn guild_icon(guild_id: GuildId, icon_hash: &str, format: &CdnResourceFormat) -> String {
        format!("https://cdn.discordapp.com/icons/{guild_id}/{icon_hash}{format}")
    }

    #[must_use]
    pub fn user_avatar(user_id: UserId, avatar_hash: &str, format: &CdnResourceFormat) -> String {
        format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar_hash}{format}")
    }
}

/// The format of a CDN resource
#[allow(clippy::module_name_repetitions)]
pub enum CdnResourceFormat {
    /// A GIF - `.gif`
    GIF,

    /// A JPEG - `.jpeg`
    JPEG,

    /// A PNG - `.png`
    PNG,

    /// A WebP - `.webp`
    WebP,
}

impl Display for CdnResourceFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GIF => f.write_str(".gif"),
            Self::JPEG => f.write_str(".jpeg"),
            Self::PNG => f.write_str(".png"),
            Self::WebP => f.write_str(".webp"),
        }
    }
}
