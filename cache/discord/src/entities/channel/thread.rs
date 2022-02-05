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

//! The thread entity.

use hartex_base::{
    discord::model::{
        channel::{
            permission_overwrite::PermissionOverwrite,
            thread::{
                AutoArchiveDuration, NewsThread, PrivateThread, PublicThread, ThreadMetadata,
            },
            ChannelType,
        },
        datetime::Timestamp,
        id::{
            marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
            Id,
        },
    },
    stdext::prelude::*,
};
use hartex_cache_base::entity::Entity;

/// A thread entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct ThreadEntity {
    default_auto_archive_duration: Option<AutoArchiveDuration>,
    guild_id: Option<Id<GuildMarker>>,
    id: Id<ChannelMarker>,
    invitable: Option<bool>,
    kind: ChannelType,
    last_message_id: Option<Id<MessageMarker>>,
    member_id: Option<Id<UserMarker>>,
    member_count: u8,
    message_count: u8,
    name: String,
    owner_id: Option<Id<UserMarker>>,
    parent_id: Option<Id<ChannelMarker>>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    rate_limit_per_user: Option<u64>,
    thread_metadata: ThreadMetadata,
}

impl ThreadEntity {
    #[must_use]
    pub fn default_auto_archive_duration(&self) -> Option<AutoArchiveDuration> {
        self.default_auto_archive_duration
    }

    #[must_use]
    pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    #[must_use]
    pub fn invitable(&self) -> Option<bool> {
        self.invitable
    }

    #[must_use]
    pub fn kind(&self) -> ChannelType {
        self.kind
    }

    #[must_use]
    pub fn last_message_id(&self) -> Option<Id<MessageMarker>> {
        self.last_message_id
    }

    #[must_use]
    pub fn member_id(&self) -> Option<Id<UserMarker>> {
        self.member_id
    }

    #[must_use]
    pub fn member_count(&self) -> u8 {
        self.member_count
    }

    #[must_use]
    pub fn message_count(&self) -> u8 {
        self.message_count
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn owner_id(&self) -> Option<Id<UserMarker>> {
        self.owner_id
    }

    #[must_use]
    pub fn parent_id(&self) -> Option<Id<ChannelMarker>> {
        self.parent_id
    }

    #[must_use]
    pub fn permission_overwrites(&self) -> Option<Vec<PermissionOverwrite>> {
        self.permission_overwrites.clone()
    }

    #[must_use]
    pub fn rate_limit_per_user(&self) -> Option<u64> {
        self.rate_limit_per_user
    }

    #[must_use]
    pub fn thread_metadata(&self) -> ThreadMetadata {
        self.thread_metadata.clone()
    }
}

impl Default for ThreadEntity {
    fn default() -> Self {
        Self {
            default_auto_archive_duration: None,
            guild_id: None,
            id: Id::new_checked(1).unwrap(),
            invitable: None,
            kind: ChannelType::GuildText,
            last_message_id: None,
            member_id: None,
            member_count: 0,
            message_count: 0,
            name: String::default(),
            owner_id: None,
            parent_id: None,
            permission_overwrites: None,
            rate_limit_per_user: None,
            thread_metadata: ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Hour,
                archive_timestamp: Timestamp::from_secs(1).expect("parsing failed"),
                create_timestamp: None,
                invitable: None,
                locked: false,
            },
        }
    }
}

impl Entity for ThreadEntity {
    type Id = Id<ChannelMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<NewsThread> for ThreadEntity {
    fn from(news_thread: NewsThread) -> Self {
        let member_id = news_thread.member.map_opt_user_id();

        Self {
            default_auto_archive_duration: news_thread.default_auto_archive_duration,
            guild_id: news_thread.guild_id,
            id: news_thread.id,
            kind: news_thread.kind,
            last_message_id: news_thread.last_message_id,
            member_id,
            member_count: news_thread.member_count,
            message_count: news_thread.message_count,
            name: news_thread.name,
            owner_id: news_thread.owner_id,
            parent_id: news_thread.parent_id,
            rate_limit_per_user: news_thread.rate_limit_per_user,
            thread_metadata: news_thread.thread_metadata,
            ..Self::default()
        }
    }
}

impl From<PrivateThread> for ThreadEntity {
    fn from(private_thread: PrivateThread) -> Self {
        let member_id = private_thread.member.map_opt_user_id();

        Self {
            default_auto_archive_duration: private_thread.default_auto_archive_duration,
            guild_id: private_thread.guild_id,
            id: private_thread.id,
            invitable: private_thread.invitable,
            kind: private_thread.kind,
            last_message_id: private_thread.last_message_id,
            member_id,
            member_count: private_thread.member_count,
            message_count: private_thread.message_count,
            name: private_thread.name,
            owner_id: private_thread.owner_id,
            parent_id: private_thread.parent_id,
            permission_overwrites: Some(private_thread.permission_overwrites),
            rate_limit_per_user: private_thread.rate_limit_per_user,
            thread_metadata: private_thread.thread_metadata,
        }
    }
}

impl From<PublicThread> for ThreadEntity {
    fn from(public_thread: PublicThread) -> Self {
        let member_id = public_thread.member.map_opt_user_id();

        Self {
            default_auto_archive_duration: public_thread.default_auto_archive_duration,
            guild_id: public_thread.guild_id,
            id: public_thread.id,
            kind: public_thread.kind,
            last_message_id: public_thread.last_message_id,
            member_id,
            member_count: public_thread.member_count,
            message_count: public_thread.message_count,
            name: public_thread.name,
            owner_id: public_thread.owner_id,
            parent_id: public_thread.parent_id,
            rate_limit_per_user: public_thread.rate_limit_per_user,
            thread_metadata: public_thread.thread_metadata,
            ..Self::default()
        }
    }
}
