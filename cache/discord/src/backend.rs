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

//! A base backend trait for different cache backends.

use hartex_cache_base::backend::Backend;

use crate::repositories::{
    channel::{
        attachment::AttachmentRepository,
        message::{
            sticker::{StickerPackRepository, StickerRepository},
            MessageRepository,
        },
        thread::ThreadRepository,
        ChannelRepository,
    },
    gateway::presence::PresenceRepository,
    guild::{
        emoji::EmojiRepository, member::MemberRepository, role::RoleRepository, GuildRepository,
    },
    user::{current_user::CurrentUserRepository, UserRepository},
};

/// A base trait for different cache backends (in-memory, database, redis, etc).
#[allow(clippy::module_name_repetitions)]
pub trait DiscordBackend: Backend + Send + Sized + Sync + 'static {
    /// The repository for attachment entities.
    type AttachmentRepository: AttachmentRepository<Self> + Send + Sync;

    /// The repository for channel entities.
    type ChannelRepository: ChannelRepository<Self> + Send + Sync;

    /// The repository for the current user entity.
    type CurrentUserRepository: CurrentUserRepository<Self> + Send + Sync;

    /// The repository for emoji entities.
    type EmojiRepository: EmojiRepository<Self> + Send + Sync;

    /// The repository for guild entities.
    type GuildRepository: GuildRepository<Self> + Send + Sync;

    /// The repository for member entities.
    type MemberRepository: MemberRepository<Self> + Send + Sync;

    /// The repository for message entities.
    type MessageRepository: MessageRepository<Self> + Send + Sync;

    /// The repository for presence entities.
    type PresenceRepository: PresenceRepository<Self> + Send + Sync;

    /// The repository for guild roles entities.
    type RoleRepository: RoleRepository<Self> + Send + Sync;

    /// The repository for user entities.
    type UserRepository: UserRepository<Self> + Send + Sync;

    /// The repository for sticker entities.
    type StickerRepository: StickerRepository<Self> + Send + Sync;

    /// The repository for sticker pack entities.
    type StickerPackRepository: StickerPackRepository<Self> + Send + Sync;

    /// The repository for thread entities.
    type ThreadRepository: ThreadRepository<Self> + Send + Sync;

    /// Returns the attachment repository of the cache.
    fn attachments(&self) -> Self::AttachmentRepository;

    /// Returns the channel repository of the cache.
    fn channels(&self) -> Self::ChannelRepository;

    /// Returns the current user repository of the cache.
    fn current_user(&self) -> Self::CurrentUserRepository;

    /// Returns the emoji repository of the cache.
    fn emojis(&self) -> Self::EmojiRepository;

    /// Returns the guild repository of the cache.
    fn guilds(&self) -> Self::GuildRepository;

    /// Returns the member repository of the cache.
    fn members(&self) -> Self::MemberRepository;

    /// Returns the message repository of the cache.
    fn messages(&self) -> Self::MessageRepository;

    /// Returns the presence repository of the cache.
    fn presences(&self) -> Self::PresenceRepository;

    /// Returns the role repository of the cache.
    fn roles(&self) -> Self::RoleRepository;

    /// Returns the user repository of the cache.
    fn users(&self) -> Self::UserRepository;

    /// Returns the sticker pack repository of the cache.
    fn sticker_packs(&self) -> Self::StickerPackRepository;

    /// Returns the sticker repository of the cache.
    fn stickers(&self) -> Self::StickerRepository;

    /// Returns the thread repository of the cache.
    fn threads(&self) -> Self::ThreadRepository;
}
