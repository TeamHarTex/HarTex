/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

use base::discord::gateway::Event;
use base::discord::model::gateway::event::shard::{
    Connected, Connecting, Disconnected, Identifying, Payload, Reconnecting, Resuming,
};
use base::discord::model::gateway::payload::incoming::{
    BanAdd, BanRemove, ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate,
    CommandPermissionsUpdate, GuildCreate, GuildDelete, GuildEmojisUpdate, GuildIntegrationsUpdate,
    GuildScheduledEventCreate, GuildScheduledEventDelete, GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd, GuildScheduledEventUserRemove, GuildStickersUpdate, GuildUpdate,
    IntegrationCreate, IntegrationDelete, IntegrationUpdate, InteractionCreate, InviteCreate,
    InviteDelete, MemberAdd, MemberChunk, MemberRemove, MemberUpdate, MessageCreate, MessageDelete,
    MessageDeleteBulk, MessageUpdate, PresenceUpdate, ReactionAdd, ReactionRemove,
    ReactionRemoveAll, ReactionRemoveEmoji, Ready, RoleCreate, RoleDelete, RoleUpdate,
    StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate, ThreadCreate, ThreadDelete,
    ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate, ThreadUpdate, TypingStart,
    UnavailableGuild, UserUpdate, VoiceServerUpdate, VoiceStateUpdate, WebhooksUpdate,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum SerdeableEvent {
    BanAdd(BanAdd),
    BanRemove(BanRemove),
    ChannelCreate(Box<ChannelCreate>),
    ChannelDelete(Box<ChannelDelete>),
    ChannelPinsUpdate(ChannelPinsUpdate),
    ChannelUpdate(Box<ChannelUpdate>),
    CommandPermissionsUpdate(CommandPermissionsUpdate),
    GatewayHeartbeat(u64),
    GatewayHeartbeatAck,
    GatewayHello(u64),
    GatewayInvalidateSession(bool),
    GatewayReconnect,
    // only provided by the Twilight library
    // is an undocumented Discord event
    // always ignored
    GiftCodeUpdate,
    GuildCreate(Box<GuildCreate>),
    GuildDelete(GuildDelete),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildScheduledEventCreate(Box<GuildScheduledEventCreate>),
    GuildScheduledEventDelete(Box<GuildScheduledEventDelete>),
    GuildScheduledEventUpdate(Box<GuildScheduledEventUpdate>),
    GuildScheduledEventUserAdd(GuildScheduledEventUserAdd),
    GuildScheduledEventUserRemove(GuildScheduledEventUserRemove),
    GuildStickersUpdate(GuildStickersUpdate),
    GuildUpdate(Box<GuildUpdate>),
    IntegrationCreate(Box<IntegrationCreate>),
    IntegrationDelete(IntegrationDelete),
    IntegrationUpdate(Box<IntegrationUpdate>),
    InteractionCreate(InteractionCreate),
    InviteCreate(Box<InviteCreate>),
    InviteDelete(InviteDelete),
    MemberAdd(Box<MemberAdd>),
    MemberChunk(MemberChunk),
    MemberRemove(MemberRemove),
    MemberUpdate(Box<MemberUpdate>),
    MessageCreate(Box<MessageCreate>),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageUpdate(Box<MessageUpdate>),
    PresenceUpdate(Box<PresenceUpdate>),
    PresencesReplace,
    ReactionAdd(Box<ReactionAdd>),
    ReactionRemove(Box<ReactionRemove>),
    ReactionRemoveAll(ReactionRemoveAll),
    ReactionRemoveEmoji(ReactionRemoveEmoji),
    Ready(Box<Ready>),
    Resumed,
    RoleCreate(RoleCreate),
    RoleDelete(RoleDelete),
    RoleUpdate(RoleUpdate),
    ShardConnected(Connected),
    ShardConnecting(Connecting),
    ShardDisconnected(Disconnected),
    ShardIdentifying(Identifying),
    ShardReconnecting(Reconnecting),
    ShardPayload(Payload),
    ShardResuming(Resuming),
    StageInstanceCreate(StageInstanceCreate),
    StageInstanceDelete(StageInstanceDelete),
    StageInstanceUpdate(StageInstanceUpdate),
    ThreadCreate(Box<ThreadCreate>),
    ThreadDelete(ThreadDelete),
    ThreadListSync(ThreadListSync),
    ThreadMemberUpdate(Box<ThreadMemberUpdate>),
    ThreadMembersUpdate(ThreadMembersUpdate),
    ThreadUpdate(Box<ThreadUpdate>),
    TypingStart(Box<TypingStart>),
    UnavailableGuild(UnavailableGuild),
    UserUpdate(UserUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    WebhooksUpdate(WebhooksUpdate),
}

impl From<Event> for SerdeableEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::BanAdd(payload) => Self::BanAdd(payload),
            Event::BanRemove(payload) => Self::BanRemove(payload),
            Event::ChannelCreate(payload) => Self::ChannelCreate(payload),
            Event::ChannelDelete(payload) => Self::ChannelDelete(payload),
            Event::ChannelPinsUpdate(payload) => Self::ChannelPinsUpdate(payload),
            Event::ChannelUpdate(payload) => Self::ChannelUpdate(payload),
            Event::CommandPermissionsUpdate(payload) => Self::CommandPermissionsUpdate(payload),
            Event::GatewayHeartbeat(payload) => Self::GatewayHeartbeat(payload),
            Event::GatewayHeartbeatAck => Self::GatewayHeartbeatAck,
            Event::GatewayHello(payload) => Self::GatewayHello(payload),
            Event::GatewayInvalidateSession(payload) => Self::GatewayInvalidateSession(payload),
            Event::GatewayReconnect => Self::GatewayReconnect,
            Event::GiftCodeUpdate => Self::GiftCodeUpdate,
            Event::GuildCreate(payload) => Self::GuildCreate(payload),
            Event::GuildDelete(payload) => Self::GuildDelete(payload),
            Event::GuildEmojisUpdate(payload) => Self::GuildEmojisUpdate(payload),
            Event::GuildIntegrationsUpdate(payload) => Self::GuildIntegrationsUpdate(payload),
            Event::GuildScheduledEventCreate(payload) => Self::GuildScheduledEventCreate(payload),
            Event::GuildScheduledEventDelete(payload) => Self::GuildScheduledEventDelete(payload),
            Event::GuildScheduledEventUpdate(payload) => Self::GuildScheduledEventUpdate(payload),
            Event::GuildScheduledEventUserAdd(payload) => Self::GuildScheduledEventUserAdd(payload),
            Event::GuildScheduledEventUserRemove(payload) => {
                Self::GuildScheduledEventUserRemove(payload)
            }
            Event::GuildStickersUpdate(payload) => Self::GuildStickersUpdate(payload),
            Event::GuildUpdate(payload) => Self::GuildUpdate(payload),
            Event::IntegrationCreate(payload) => Self::IntegrationCreate(payload),
            Event::IntegrationDelete(payload) => Self::IntegrationDelete(payload),
            Event::IntegrationUpdate(payload) => Self::IntegrationUpdate(payload),
            Event::InteractionCreate(payload) => Self::InteractionCreate(payload),
            Event::InviteCreate(payload) => Self::InviteCreate(payload),
            Event::InviteDelete(payload) => Self::InviteDelete(payload),
            Event::MemberAdd(payload) => Self::MemberAdd(payload),
            Event::MemberChunk(payload) => Self::MemberChunk(payload),
            Event::MemberRemove(payload) => Self::MemberRemove(payload),
            Event::MemberUpdate(payload) => Self::MemberUpdate(payload),
            Event::MessageCreate(payload) => Self::MessageCreate(payload),
            Event::MessageDelete(payload) => Self::MessageDelete(payload),
            Event::MessageDeleteBulk(payload) => Self::MessageDeleteBulk(payload),
            Event::MessageUpdate(payload) => Self::MessageUpdate(payload),
            Event::PresenceUpdate(payload) => Self::PresenceUpdate(payload),
            Event::PresencesReplace => Self::PresencesReplace,
            Event::ReactionAdd(payload) => Self::ReactionAdd(payload),
            Event::ReactionRemove(payload) => Self::ReactionRemove(payload),
            Event::ReactionRemoveAll(payload) => Self::ReactionRemoveAll(payload),
            Event::ReactionRemoveEmoji(payload) => Self::ReactionRemoveEmoji(payload),
            Event::Ready(payload) => Self::Ready(payload),
            Event::Resumed => Self::Resumed,
            Event::RoleCreate(payload) => Self::RoleCreate(payload),
            Event::RoleDelete(payload) => Self::RoleDelete(payload),
            Event::RoleUpdate(payload) => Self::RoleUpdate(payload),
            Event::ShardConnected(payload) => Self::ShardConnected(payload),
            Event::ShardConnecting(payload) => Self::ShardConnecting(payload),
            Event::ShardDisconnected(payload) => Self::ShardDisconnected(payload),
            Event::ShardIdentifying(payload) => Self::ShardIdentifying(payload),
            Event::ShardReconnecting(payload) => Self::ShardReconnecting(payload),
            Event::ShardPayload(payload) => Self::ShardPayload(payload),
            Event::ShardResuming(payload) => Self::ShardResuming(payload),
            Event::StageInstanceCreate(payload) => Self::StageInstanceCreate(payload),
            Event::StageInstanceDelete(payload) => Self::StageInstanceDelete(payload),
            Event::StageInstanceUpdate(payload) => Self::StageInstanceUpdate(payload),
            Event::ThreadCreate(payload) => Self::ThreadCreate(payload),
            Event::ThreadDelete(payload) => Self::ThreadDelete(payload),
            Event::ThreadListSync(payload) => Self::ThreadListSync(payload),
            Event::ThreadMemberUpdate(payload) => Self::ThreadMemberUpdate(payload),
            Event::ThreadMembersUpdate(payload) => Self::ThreadMembersUpdate(payload),
            Event::ThreadUpdate(payload) => Self::ThreadUpdate(payload),
            Event::TypingStart(payload) => Self::TypingStart(payload),
            Event::UnavailableGuild(payload) => Self::UnavailableGuild(payload),
            Event::UserUpdate(payload) => Self::UserUpdate(payload),
            Event::VoiceServerUpdate(payload) => Self::VoiceServerUpdate(payload),
            Event::VoiceStateUpdate(payload) => Self::VoiceStateUpdate(payload),
            Event::WebhooksUpdate(payload) => Self::WebhooksUpdate(payload),
        }
    }
}
