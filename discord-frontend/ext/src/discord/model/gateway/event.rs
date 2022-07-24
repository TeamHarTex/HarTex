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

//! Extensions to the [`Event`] type in `twilight-model`.

use base::discord::model::gateway::event::Event;

#[allow(clippy::module_name_repetitions)]
pub trait EventExt {
    fn as_str(&self) -> &str;
}

impl EventExt for Event {
    fn as_str(&self) -> &str {
        match self {
            Event::AutoModerationActionExecution(_) => "AUTO_MODERATION_ACTION_EXECUTION",
            Event::AutoModerationRuleCreate(_) => "AUTO_MODERATION_RULE_CREATE",
            Event::AutoModerationRuleDelete(_) => "AUTO_MODERATION_RULE_DELETE",
            Event::AutoModerationRuleUpdate(_) => "AUTO_MODERATION_RULE_UPDATE",
            Event::BanAdd(_) => "GUILD_BAN_ADD",
            Event::BanRemove(_) => "GUILD_BAN_REMOVE",
            Event::ChannelCreate(_) => "CHANNEL_CREATE",
            Event::ChannelDelete(_) => "CHANNEL_DELETE",
            Event::ChannelUpdate(_) => "CHANNEL_UPDATE",
            Event::ChannelPinsUpdate(_) => "CHANNEL_PINS_UPDATE",
            Event::CommandPermissionsUpdate(_) => "COMMAND_PERMISSIONS_UPDATE",
            Event::GatewayHeartbeat(_) => "HEARTBEAT",
            Event::GatewayHeartbeatAck => "GATEWAY_HEARTBEAT_ACK",
            Event::GatewayHello(_) => "HELLO",
            Event::GatewayInvalidateSession(_) => "GATEWAY_INVALID_SESSION",
            Event::GatewayReconnect => "RECONNECT",
            Event::GiftCodeUpdate => "GIFT_CODE_UPDATE",
            Event::GuildCreate(_) => "GUILD_CREATE",
            Event::GuildDelete(_) => "GUILD_DELETE",
            Event::GuildEmojisUpdate(_) => "GUILD_EMOJIS_UPDATE",
            Event::GuildIntegrationsUpdate(_) => "GUILD_INTEGRATIONS_UPDATE",
            Event::GuildScheduledEventCreate(_) => "GUILD_SCHEDULED_EVENT_CREATE",
            Event::GuildScheduledEventDelete(_) => "GUILD_SCHEDULED_EVENT_DELETE",
            Event::GuildScheduledEventUpdate(_) => "GUILD_SCHEDULED_EVENT_UPDATE",
            Event::GuildScheduledEventUserAdd(_) => "GUILD_SCHEDULED_EVENT_USER_ADD",
            Event::GuildScheduledEventUserRemove(_) => "GUILD_SCHEDULED_EVENT_USER_REMOVE",
            Event::GuildStickersUpdate(_) => "GUILD_STICKERS_UPDATE",
            Event::GuildUpdate(_) => "GUILD_UPDATE",
            Event::IntegrationCreate(_) => "INTEGRATION_CREATE",
            Event::IntegrationDelete(_) => "INTEGRATION_DELETE",
            Event::IntegrationUpdate(_) => "INTEGRATION_UPDATE",
            Event::InteractionCreate(_) => "INTERACTION_CREATE",
            Event::InviteCreate(_) => "INVITE_CREATE",
            Event::InviteDelete(_) => "INVITE_DELETE",
            Event::MemberAdd(_) => "GUILD_MEMBER_ADD",
            Event::MemberRemove(_) => "GUILD_MEMBER_REMOVE",
            Event::MemberUpdate(_) => "GUILD_MEMBER_UPDATE",
            Event::MemberChunk(_) => "GUILD_MEMBERS_CHUNK",
            Event::MessageCreate(_) => "MESSAGE_CREATE",
            Event::MessageDelete(_) => "MESSAGE_DELETE",
            Event::MessageDeleteBulk(_) => "MESSAGE_DELETE_BULK",
            Event::MessageUpdate(_) => "MESSAGE_UPDATE",
            Event::PresencesReplace => "PRESENCES_REPLACE",
            Event::PresenceUpdate(_) => "PRESENCE_UPDATE",
            Event::ReactionAdd(_) => "MESSAGE_REACTION_ADD",
            Event::ReactionRemove(_) => "MESSAGE_REACTION_REMOVE",
            Event::ReactionRemoveAll(_) => "MESSAGE_REACTION_REMOVE_ALL",
            Event::ReactionRemoveEmoji(_) => "MESSAGE_REACTION_REMOVE_EMOJI",
            Event::Ready(_) => "READY",
            Event::Resumed => "RESUMED",
            Event::RoleCreate(_) => "GUILD_ROLE_CREATE",
            Event::RoleDelete(_) => "GUILD_ROLE_DELETE",
            Event::RoleUpdate(_) => "GUILD_ROLE_UPDATE",
            Event::ShardConnected(_) => "SHARD_CONNECTED",
            Event::ShardConnecting(_) => "SHARD_CONNECTING",
            Event::ShardDisconnected(_) => "SHARD_DISCONNECTED",
            Event::ShardIdentifying(_) => "SHARD_IDENTIFYING",
            Event::ShardReconnecting(_) => "SHARD_RECONNECTING",
            Event::ShardPayload(_) => "SHARD_PAYLOAD",
            Event::ShardResuming(_) => "SHARD_RESUMING",
            Event::StageInstanceCreate(_) => "STAGE_INSTANCE_CREATE",
            Event::StageInstanceDelete(_) => "STAGE_INSTANCE_DELETE",
            Event::StageInstanceUpdate(_) => "STAGE_INSTANCE_UPDATE",
            Event::ThreadCreate(_) => "THREAD_CREATE",
            Event::ThreadDelete(_) => "THREAD_DELETE",
            Event::ThreadListSync(_) => "THREAD_LIST_SYNC",
            Event::ThreadMemberUpdate(_) => "THREAD_MEMBER_UPDATE",
            Event::ThreadMembersUpdate(_) => "THREAD_MEMBERS_UPDATE",
            Event::ThreadUpdate(_) => "THREAD_UPDATE",
            Event::TypingStart(_) => "TYPING_START",
            Event::UserUpdate(_) => "USER_UPDATE",
            Event::VoiceServerUpdate(_) => "VOICE_SERVER_UPDATE",
            Event::VoiceStateUpdate(_) => "VOICE_STATE_UPDATE",
            Event::WebhooksUpdate(_) => "WEBHOOKS_UPDATE",
            Event::UnavailableGuild(_) => "UNAVAILABLE_GUILD",
        }
    }
}
