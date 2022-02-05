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

//! The guild member repository trait.

use hartex_base::discord::model::id::{
    GuildId,
    UserId
};
use hartex_cache_base::{
    relations,
    repository::{
        GetEntityFuture,
        Repository,
        StreamEntitiesFuture
    }
};

use crate::{
    backend::DiscordBackend,
    entities::{
        guild::{
            member::MemberEntity,
            role::RoleEntity
        },
        user::UserEntity
    }
};

/// A repository containing member objects.
#[allow(clippy::module_name_repetitions)]
pub trait MemberRepository<B: DiscordBackend>: Repository<MemberEntity, B> {
    /// A stream of roles of a member.
    fn roles(
        &self,
        guild_id: GuildId,
        user_id: UserId
    ) -> StreamEntitiesFuture<'_, RoleEntity, B::Error>;

    /// The associated user of the member.
    fn user(
        &self,
        guild_id: GuildId,
        user_id: UserId
    ) -> GetEntityFuture<'_, UserEntity, B::Error> {
        let backend = self.backend();

        relations::map_entity(
            backend.members(),
            backend.users(),
            (guild_id, user_id),
            |member| member.user_id()
        )
    }
}
