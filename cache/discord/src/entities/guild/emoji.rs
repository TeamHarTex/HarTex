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

//! # The `emoji` Module
//!
//! This module implements the guild emoji entity.

use hartex_base::discord::model::{
    guild::Emoji,
    id::{
        EmojiId,
        GuildId,
        RoleId,
        UserId
    }
};

use crate::entity::Entity;

/// # Struct `EmojiEntity`
///
/// An emoji entity.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct EmojiEntity {
    animated: bool,
    available: bool,
    guild_id: GuildId,
    id: EmojiId,
    managed: bool,
    name: String,
    require_colons: bool,
    role_ids: Vec<RoleId>,
    user_id: Option<UserId>
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
    pub fn guild_id(&self) -> GuildId {
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
    pub fn role_ids(&self) -> Vec<RoleId> {
        self.role_ids.clone()
    }

    #[must_use]
    pub fn user_id(&self) -> Option<UserId> {
        self.user_id
    }
}

impl Entity for EmojiEntity {
    type Id = EmojiId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<(GuildId, Emoji)> for EmojiEntity {
    fn from((guild_id, emoji): (GuildId, Emoji)) -> Self {
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
            user_id
        }
    }
}
