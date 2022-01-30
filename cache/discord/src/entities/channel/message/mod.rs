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

//! Entities related to Discord channel messages.

use hartex_base::{
    discord::model::{
        application::component::Component,
        channel::{
            embed::Embed,
            message::{
                sticker::MessageSticker,
                Mention,
                Message,
                MessageActivity,
                MessageApplication,
                MessageFlags,
                MessageInteraction,
                MessageReaction,
                MessageReference,
                MessageType
            },
            ChannelMention
        },
        datetime::Timestamp,
        id::{
            ApplicationId,
            AttachmentId,
            ChannelId,
            GuildId,
            MessageId,
            RoleId,
            UserId,
            WebhookId
        }
    },
    stdext::prelude::*
};
use hartex_cache_base::entity::Entity;

pub mod sticker;

/// A message entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct MessageEntity {
    activity: Option<MessageActivity>,
    application: Option<MessageApplication>,
    application_id: Option<ApplicationId>,
    attachment_ids: Vec<AttachmentId>,
    author_id: UserId,
    channel_id: ChannelId,
    components: Vec<Component>,
    content: String,
    edited_timestamp: Option<Timestamp>,
    embeds: Vec<Embed>,
    flags: Option<MessageFlags>,
    guild_id: Option<GuildId>,
    id: MessageId,
    interaction: Option<MessageInteraction>,
    kind: MessageType,
    member_id: Option<UserId>,
    mention_channels: Vec<ChannelMention>,
    mention_everyone: bool,
    mention_roles: Vec<RoleId>,
    mentions: Vec<Mention>,
    pinned: bool,
    reactions: Vec<MessageReaction>,
    reference: Option<MessageReference>,
    referenced_message_id: Option<MessageId>,
    sticker_items: Vec<MessageSticker>,
    timestamp: Timestamp,
    thread_id: Option<ChannelId>,
    tts: bool,
    webhook_id: Option<WebhookId>
}

impl MessageEntity {
    #[must_use]
    pub fn activity(&self) -> Option<MessageActivity> {
        self.activity.clone()
    }

    #[must_use]
    pub fn application(&self) -> Option<MessageApplication> {
        self.application.clone()
    }

    #[must_use]
    pub fn application_id(&self) -> Option<ApplicationId> {
        self.application_id
    }

    #[must_use]
    pub fn attachment_ids(&self) -> Vec<AttachmentId> {
        self.attachment_ids.clone()
    }

    #[must_use]
    pub fn author_id(&self) -> UserId {
        self.author_id
    }

    #[must_use]
    pub fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    #[must_use]
    pub fn components(&self) -> Vec<Component> {
        self.components.clone()
    }

    #[must_use]
    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    #[must_use]
    pub fn edited_timestamp(&self) -> Option<Timestamp> {
        self.edited_timestamp
    }

    #[must_use]
    pub fn embed(&self) -> Vec<Embed> {
        self.embeds.clone()
    }

    #[must_use]
    pub fn flags(&self) -> Option<MessageFlags> {
        self.flags
    }

    #[must_use]
    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    #[must_use]
    pub fn interaction(&self) -> Option<MessageInteraction> {
        self.interaction.clone()
    }

    #[must_use]
    pub fn kind(&self) -> MessageType {
        self.kind
    }

    #[must_use]
    pub fn member_id(&self) -> Option<UserId> {
        self.member_id
    }

    #[must_use]
    pub fn mention_channels(&self) -> Vec<ChannelMention> {
        self.mention_channels.clone()
    }

    #[must_use]
    pub fn mention_everyone(&self) -> bool {
        self.mention_everyone
    }

    #[must_use]
    pub fn mention_roles(&self) -> Vec<RoleId> {
        self.mention_roles.clone()
    }

    #[must_use]
    pub fn mentions(&self) -> Vec<Mention> {
        self.mentions.clone()
    }

    #[must_use]
    pub fn pinned(&self) -> bool {
        self.pinned
    }

    #[must_use]
    pub fn reactions(&self) -> Vec<MessageReaction> {
        self.reactions.clone()
    }

    #[must_use]
    pub fn reference(&self) -> Option<MessageReference> {
        self.reference.clone()
    }

    #[must_use]
    pub fn referenced_message_id(&self) -> Option<MessageId> {
        self.referenced_message_id
    }

    #[must_use]
    pub fn sticker_items(&self) -> Vec<MessageSticker> {
        self.sticker_items.clone()
    }

    #[must_use]
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    #[must_use]
    pub fn thread_id(&self) -> Option<ChannelId> {
        self.thread_id
    }

    #[must_use]
    pub fn tts(&self) -> bool {
        self.tts
    }

    #[must_use]
    pub fn webhook_id(&self) -> Option<WebhookId> {
        self.webhook_id
    }
}

impl Entity for MessageEntity {
    type Id = MessageId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<Message> for MessageEntity {
    #[allow(unreachable_code)] // temporarily allow unreachable code until thread_id is implemented
    fn from(message: Message) -> Self {
        let attachment_ids = message
            .attachments
            .iter()
            .map(|attachment| attachment.id)
            .collect();
        let member_id = message.member.map_opt_user_id();
        let referenced_message_id = message.referenced_message.map(|message| message.id);

        Self {
            activity: message.activity,
            application: message.application,
            application_id: message.application_id,
            attachment_ids,
            author_id: message.author.id,
            channel_id: message.channel_id,
            components: message.components,
            content: message.content,
            edited_timestamp: message.edited_timestamp,
            embeds: message.embeds,
            flags: message.flags,
            guild_id: message.guild_id,
            id: message.id,
            interaction: message.interaction,
            kind: message.kind,
            member_id,
            mention_channels: message.mention_channels,
            mention_everyone: message.mention_everyone,
            mention_roles: message.mention_roles,
            mentions: message.mentions,
            pinned: message.pinned,
            reactions: message.reactions,
            reference: message.reference,
            referenced_message_id,
            sticker_items: message.sticker_items,
            timestamp: message.timestamp,
            thread_id: todo!(),
            tts: message.tts,
            webhook_id: message.webhook_id
        }
    }
}
