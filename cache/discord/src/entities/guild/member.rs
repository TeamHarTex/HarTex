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

//! The guild member entity.

use hartex_base::{
    discord::model::{
        datetime::Timestamp,
        guild::Member,
        id::{
            GuildId,
            RoleId,
            UserId
        }
    },
    stdext::prelude::*
};
use hartex_cache_base::entity::Entity;

/// A member entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct MemberEntity {
    avatar: Option<String>,
    communication_disabled_until: Option<Timestamp>,
    deaf: bool,
    guild_id: GuildId,
    joined_at: Timestamp,
    mute: bool,
    nick: Option<String>,
    pending: bool,
    premium_since: Option<Timestamp>,
    role_ids: Vec<RoleId>,
    user_id: UserId
}

impl MemberEntity {
    #[must_use]
    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_refstr()
    }

    #[must_use]
    pub fn communication_disabled_until(&self) -> Option<Timestamp> {
        self.communication_disabled_until
    }

    #[must_use]
    pub fn deaf(&self) -> bool {
        self.deaf
    }

    #[must_use]
    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn joined_at(&self) -> Timestamp {
        self.joined_at
    }

    #[must_use]
    pub fn mute(&self) -> bool {
        self.mute
    }

    #[must_use]
    pub fn nick(&self) -> Option<&str> {
        self.nick.as_refstr()
    }

    #[must_use]
    pub fn pending(&self) -> bool {
        self.pending
    }

    #[must_use]
    pub fn premium_since(&self) -> Option<Timestamp> {
        self.premium_since
    }

    #[must_use]
    pub fn role_ids(&self) -> Vec<RoleId> {
        self.role_ids.clone()
    }

    #[must_use]
    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}

impl Entity for MemberEntity {
    type Id = (GuildId, UserId);

    fn id(&self) -> Self::Id {
        (self.guild_id, self.user_id)
    }
}

impl From<Member> for MemberEntity {
    fn from(member: Member) -> Self {
        Self {
            avatar: member.avatar,
            communication_disabled_until: member.communication_disabled_until,
            deaf: member.deaf,
            guild_id: member.guild_id,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick,
            pending: member.pending,
            premium_since: member.premium_since,
            role_ids: member.roles,
            user_id: member.user.id
        }
    }
}
