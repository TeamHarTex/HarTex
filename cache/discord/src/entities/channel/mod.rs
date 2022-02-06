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

//! Entities related to Discord channels.

use hartex_base::{
    discord::model::{
        channel::{
            permission_overwrite::PermissionOverwrite, CategoryChannel, ChannelType, Group,
            PrivateChannel, TextChannel, VideoQualityMode, VoiceChannel,
        },
        datetime::Timestamp,
        id::{
            marker::{ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, UserMarker},
            Id,
        },
        util::ImageHash,
    },
    stdext::prelude::*,
};
use hartex_cache_base::entity::Entity;

pub mod attachment;
pub mod message;
pub mod thread;

/// A channel entity of any type.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct ChannelEntity {
    application_id: Option<Id<ApplicationMarker>>,
    bitrate: Option<u64>,
    guild_id: Option<Id<GuildMarker>>,
    icon: Option<ImageHash>,
    id: Id<ChannelMarker>,
    kind: ChannelType,
    last_message_id: Option<Id<MessageMarker>>,
    last_pin_timestamp: Option<Timestamp>,
    name: Option<String>,
    nsfw: Option<bool>,
    owner_id: Option<Id<UserMarker>>,
    parent_id: Option<Id<ChannelMarker>>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<i64>,
    rate_limit_per_user: Option<u64>,
    recipient_ids: Option<Vec<Id<UserMarker>>>,
    rtc_region: Option<String>,
    topic: Option<String>,
    user_limit: Option<u64>,
    video_quality_mode: Option<VideoQualityMode>,
}

impl ChannelEntity {
    #[must_use]
    pub fn application_id(&self) -> Option<Id<ApplicationMarker>> {
        self.application_id
    }

    #[must_use]
    pub fn bitrate(&self) -> Option<u64> {
        self.bitrate
    }

    #[must_use]
    pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    #[must_use]
    pub fn icon(&self) -> Option<ImageHash> {
        self.icon
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
    pub fn last_pin_timestamp(&self) -> Option<Timestamp> {
        self.last_pin_timestamp
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_refstr()
    }

    #[must_use]
    pub fn nsfw(&self) -> Option<bool> {
        self.nsfw
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
    pub fn position(&self) -> Option<i64> {
        self.position
    }

    #[must_use]
    pub fn rate_limit_per_user(&self) -> Option<u64> {
        self.rate_limit_per_user
    }

    #[must_use]
    pub fn recipient_ids(&self) -> Option<Vec<Id<UserMarker>>> {
        self.recipient_ids.clone()
    }

    #[must_use]
    pub fn rtc_region(&self) -> Option<&str> {
        self.rtc_region.as_refstr()
    }

    #[must_use]
    pub fn topic(&self) -> Option<&str> {
        self.topic.as_refstr()
    }

    #[must_use]
    pub fn user_limit(&self) -> Option<u64> {
        self.user_limit
    }

    #[must_use]
    pub fn video_quality_mode(&self) -> Option<VideoQualityMode> {
        self.video_quality_mode
    }
}

impl Default for ChannelEntity {
    fn default() -> Self {
        Self {
            application_id: None,
            bitrate: None,
            guild_id: None,
            icon: None,
            id: Id::new_checked(1).unwrap(),
            kind: ChannelType::Group,
            last_message_id: None,
            last_pin_timestamp: None,
            name: None,
            nsfw: None,
            owner_id: None,
            parent_id: None,
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: None,
            recipient_ids: None,
            rtc_region: None,
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        }
    }
}

impl Entity for ChannelEntity {
    type Id = Id<ChannelMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<CategoryChannel> for ChannelEntity {
    fn from(category_channel: CategoryChannel) -> Self {
        Self {
            guild_id: category_channel.guild_id,
            id: category_channel.id,
            kind: category_channel.kind,
            name: Some(category_channel.name),
            permission_overwrites: Some(category_channel.permission_overwrites),
            position: Some(category_channel.position),
            ..Self::default()
        }
    }
}

impl From<Group> for ChannelEntity {
    fn from(group: Group) -> Self {
        let recipient_ids = Some(group.recipients.iter().map(|user| user.id).collect());

        Self {
            application_id: group.application_id,
            icon: group.icon,
            id: group.id,
            kind: group.kind,
            last_message_id: group.last_message_id,
            last_pin_timestamp: group.last_pin_timestamp,
            name: group.name,
            owner_id: Some(group.owner_id),
            recipient_ids,
            ..Self::default()
        }
    }
}

impl From<PrivateChannel> for ChannelEntity {
    fn from(private_channel: PrivateChannel) -> Self {
        let recipient_ids = Some(
            private_channel
                .recipients
                .iter()
                .map(|user| user.id)
                .collect(),
        );

        Self {
            id: private_channel.id,
            last_message_id: private_channel.last_message_id,
            last_pin_timestamp: private_channel.last_pin_timestamp,
            kind: private_channel.kind,
            recipient_ids,
            ..Self::default()
        }
    }
}

impl From<TextChannel> for ChannelEntity {
    fn from(text_channel: TextChannel) -> Self {
        Self {
            guild_id: text_channel.guild_id,
            id: text_channel.id,
            kind: text_channel.kind,
            last_message_id: text_channel.last_message_id,
            last_pin_timestamp: text_channel.last_pin_timestamp,
            name: Some(text_channel.name),
            nsfw: Some(text_channel.nsfw),
            parent_id: text_channel.parent_id,
            permission_overwrites: Some(text_channel.permission_overwrites),
            position: Some(text_channel.position),
            rate_limit_per_user: text_channel.rate_limit_per_user,
            topic: text_channel.topic,
            ..Self::default()
        }
    }
}

impl From<VoiceChannel> for ChannelEntity {
    fn from(voice_channel: VoiceChannel) -> Self {
        Self {
            bitrate: Some(voice_channel.bitrate),
            guild_id: voice_channel.guild_id,
            id: voice_channel.id,
            kind: voice_channel.kind,
            name: Some(voice_channel.name),
            parent_id: voice_channel.parent_id,
            permission_overwrites: Some(voice_channel.permission_overwrites),
            position: Some(voice_channel.position),
            rtc_region: voice_channel.rtc_region,
            user_limit: voice_channel.user_limit,
            video_quality_mode: voice_channel.video_quality_mode,
            ..Self::default()
        }
    }
}
