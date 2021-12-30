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
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `inmemory` Module
//!
//! This module contains the in-memory backend for the cache.

use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    },
    marker::PhantomData,
    sync::{
        Arc,
        Mutex
    }
};

use dashmap::{
    DashMap,
    DashSet
};
use hartex_base::discord::model::id::{
    AttachmentId,
    EmojiId,
    GuildId,
    MessageId,
    RoleId,
    UserId
};

use crate::{
    backend::Backend,
    entities::{
        channel::attachment::AttachmentEntity,
        guild::{
            emoji::EmojiEntity,
            member::MemberEntity,
            role::RoleEntity,
            GuildEntity
        },
        user::{
            current_user::CurrentUserEntity,
            UserEntity
        }
    },
    entity::Entity,
    inmemory::repository::InMemoryRepository
};

pub mod repository;

/// # Struct `InMemoryBackend`
///
/// In-memory backend for the cache.
#[derive(Clone)]
pub struct InMemoryBackend(Arc<InMemoryBackendRef>);

impl InMemoryBackend {
    fn repository<T: Entity>(&self) -> InMemoryRepository<T> {
        InMemoryRepository(self.clone(), PhantomData)
    }
}

impl Backend for InMemoryBackend {
    type Error = InMemoryBackendError;
    type AttachmentRepository = InMemoryRepository<AttachmentEntity>;
    type CurrentUserRepository = InMemoryRepository<CurrentUserEntity>;
    type EmojiRepository = InMemoryRepository<EmojiEntity>;
    type GuildRepository = InMemoryRepository<GuildEntity>;
    type MemberRepository = InMemoryRepository<MemberEntity>;
    type RoleRepository = InMemoryRepository<RoleEntity>;
    type UserRepository = InMemoryRepository<UserEntity>;

    fn attachments(&self) -> Self::AttachmentRepository {
        self.repository::<AttachmentEntity>()
    }

    fn current_user(&self) -> Self::CurrentUserRepository {
        self.repository::<CurrentUserEntity>()
    }

    fn emojis(&self) -> Self::EmojiRepository {
        self.repository::<EmojiEntity>()
    }

    fn guilds(&self) -> Self::GuildRepository {
        self.repository::<GuildEntity>()
    }

    fn members(&self) -> Self::MemberRepository {
        self.repository::<MemberEntity>()
    }

    fn roles(&self) -> Self::RoleRepository {
        self.repository::<RoleEntity>()
    }

    fn users(&self) -> Self::UserRepository {
        self.repository::<UserEntity>()
    }
}

/// # Struct `InMemoryBackendError`
///
/// Error returned from backend operations.
#[derive(Clone, Debug)]
pub struct InMemoryBackendError;

impl Display for InMemoryBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "this error cannot be created")
    }
}

impl Error for InMemoryBackendError {}

struct InMemoryBackendRef {
    attachments: DashMap<(MessageId, AttachmentId), AttachmentEntity>,
    current_user: Mutex<Option<CurrentUserEntity>>,
    emojis: DashMap<EmojiId, EmojiEntity>,
    guilds: DashMap<GuildId, GuildEntity>,
    guild_emojis: DashMap<GuildId, DashSet<EmojiId>>,
    guild_members: DashMap<GuildId, DashSet<UserId>>,
    guild_roles: DashMap<GuildId, DashSet<RoleId>>,
    members: DashMap<(GuildId, UserId), MemberEntity>,
    roles: DashMap<RoleId, RoleEntity>,
    users: DashMap<UserId, UserEntity>
}
