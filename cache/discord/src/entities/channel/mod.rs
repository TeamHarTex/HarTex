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

//! # The `channel` Module
//!
//! This module contains entities related to Discord channels.

use hartex_base::discord::model::{
    channel::{
        thread::{
            AutoArchiveDuration,
            NewsThread,
            PrivateThread,
            PublicThread,
            ThreadMetadata
        },
        permission_overwrite::PermissionOverwrite,
        CategoryChannel,
        ChannelType,
        Group,
        PrivateChannel,
        TextChannel,
        VoiceChannel,
        VideoQualityMode
    },
    datetime::Timestamp,
    id::{
        ApplicationId,
        ChannelId,
        GuildId,
        MessageId,
        UserId
    }
};

use crate::entity::Entity;

pub mod attachment;
pub mod message;

/// # Struct `ChannelEntity`
///
/// A channel entity of any type.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct ChannelEntity {
    application_id: Option<ApplicationId>,
    bitrate: Option<u64>,
    guild_id: Option<GuildId>,
    icon: Option<String>,
    id: ChannelId,
    kind: ChannelType,
    last_message_id: Option<MessageId>,
    last_pin_timestamp: Option<Timestamp>,
    name: Option<String>,
    nsfw: Option<bool>,
    owner_id: Option<UserId>,
    parent_id: Option<ChannelId>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<i64>,
    rate_limit_per_user: Option<u64>,
    recipient_ids: Option<Vec<UserId>>,
    rtc_region: Option<String>,
    topic: Option<String>,
    user_limit: Option<u64>,
    video_quality_mode: Option<VideoQualityMode>
}

impl Entity for ChannelEntity {
    type Id = ChannelId;

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
            ..Default::default()
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
            ..Default::default()
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
                .collect()
        );

        Self {
            id: private_channel.id,
            last_message_id: private_channel.last_message_id,
            last_pin_timestamp: private_channel.last_pin_timestamp,
            kind: private_channel.kind,
            recipient_ids,
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        }
    }
}
