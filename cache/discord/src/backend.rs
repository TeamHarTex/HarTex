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

//! # The `backend` Module
//!
//! This module contains a base backend trait for different cache backends.

use crate::repositories::{
    channel::{
        attachment::AttachmentRepository,
        message::{
            sticker::{
                StickerPackRepository,
                StickerRepository
            },
            MessageRepository
        }
    },
    guild::{
        emoji::EmojiRepository,
        member::MemberRepository,
        role::RoleRepository,
        GuildRepository
    },
    user::{
        current_user::CurrentUserRepository,
        UserRepository
    }
};

/// # Trait `Backend`
///
/// A base trait for different cache backends (in-memory, database, redis, etc).
pub trait Backend: Send + Sized + Sync + 'static {
    /// # Typealias `Error`
    ///
    /// The backend error type.
    type Error: Send + 'static;

    /// # Typealias `AttachmentRepository`
    ///
    /// The repository for attachment entities.
    type AttachmentRepository: AttachmentRepository<Self> + Send + Sync;

    /// # Typealias `CurrentUserRepository`
    ///
    /// The repository for the current user entity.
    type CurrentUserRepository: CurrentUserRepository<Self> + Send + Sync;

    /// # Typealias `EmojiRepository`
    ///
    /// The repository for emoji entities.
    type EmojiRepository: EmojiRepository<Self> + Send + Sync;

    /// # Typealias `GuildRepository`
    ///
    /// The repository for guild entities.
    type GuildRepository: GuildRepository<Self> + Send + Sync;

    /// # Typealias `MemberRepository`
    ///
    /// The repository for member entities.
    type MemberRepository: MemberRepository<Self> + Send + Sync;

    /// # Typealias `MessageRepository`
    ///
    /// The repository for message entities.
    type MessageRepository: MessageRepository<Self> + Send + Sync;

    /// # Typealias `RoleRepository`
    ///
    /// The repository for guild roles entities.
    type RoleRepository: RoleRepository<Self> + Send + Sync;

    /// # Typealias `UserRepository`
    ///
    /// The repository for user entities.
    type UserRepository: UserRepository<Self> + Send + Sync;

    /// # Typealias `StickerRepository`
    ///
    /// The repository for sticker entities.
    type StickerRepository: StickerRepository<Self> + Send + Sync;

    /// # Typealias `StickerPackRepository`
    ///
    /// The repository for sticker pack entities.
    type StickerPackRepository: StickerPackRepository<Self> + Send + Sync;

    /// # Trait Method `attachments`
    ///
    /// Returns the attachment repository of the cache.
    fn attachments(&self) -> Self::AttachmentRepository;

    /// # Trait Method `current_user`
    ///
    /// Returns the current user repository of the cache.
    fn current_user(&self) -> Self::CurrentUserRepository;

    /// # Trait Method `emojis`
    ///
    /// Returns the emoji repository of the cache.
    fn emojis(&self) -> Self::EmojiRepository;

    /// # Trait Method `guilds`
    ///
    /// Returns the guild repository of the cache.
    fn guilds(&self) -> Self::GuildRepository;

    /// # Trait Method `members`
    ///
    /// Returns the member repository of the cache.
    fn members(&self) -> Self::MemberRepository;

    /// # Trait Method `messages`
    ///
    /// Returns the message repository of the cache.
    fn messages(&self) -> Self::MessageRepository;

    /// # Trait Method `roles`
    ///
    /// Returns the role repository of the cache.
    fn roles(&self) -> Self::RoleRepository;

    /// # Trait Method `users`
    ///
    /// Returns the user repository of the cache.
    fn users(&self) -> Self::UserRepository;

    /// # Trait Method `sticker_packs`
    ///
    /// Returns the sticker pack repository of the cache.
    fn sticker_packs(&self) -> Self::StickerPackRepository;

    /// # Trait Method `stickers`
    ///
    /// Returns the sticker repository of the cache.
    fn stickers(&self) -> Self::StickerRepository;
}
