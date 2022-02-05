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

//! The guild emoji entity.

use hartex_base::discord::model::{
    guild::Emoji,
    id::{
        marker::{EmojiMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
use hartex_cache_base::entity::Entity;

/// An emoji entity.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct EmojiEntity {
    animated: bool,
    available: bool,
    guild_id: Id<GuildMarker>,
    id: Id<EmojiMarker>,
    managed: bool,
    name: String,
    require_colons: bool,
    role_ids: Vec<Id<RoleMarker>>,
    user_id: Option<Id<UserMarker>>,
}

impl EmojiEntity {
    #[must_use]
    pub fn animated(&self) -> bool {
        self.animated
    }

    #[must_use]
    pub fn available(&self) -> bool {
        self.available
    }

    #[must_use]
    pub fn guild_id(&self) -> Id<GuildMarker> {
        self.guild_id
    }

    #[must_use]
    pub fn managed(&self) -> bool {
        self.managed
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn require_colons(&self) -> bool {
        self.require_colons
    }

    #[must_use]
    pub fn role_ids(&self) -> Vec<Id<RoleMarker>> {
        self.role_ids.clone()
    }

    #[must_use]
    pub fn user_id(&self) -> Option<Id<UserMarker>> {
        self.user_id
    }
}

impl Entity for EmojiEntity {
    type Id = Id<EmojiMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<(Id<GuildMarker>, Emoji)> for EmojiEntity {
    fn from((guild_id, emoji): (Id<GuildMarker>, Emoji)) -> Self {
        let user_id = emoji.user.map(|user| user.id);

        Self {
            animated: emoji.animated,
            available: emoji.available,
            guild_id,
            id: emoji.id,
            managed: emoji.managed,
            name: emoji.name,
            require_colons: emoji.require_colons,
            role_ids: emoji.roles,
            user_id,
        }
    }
}
