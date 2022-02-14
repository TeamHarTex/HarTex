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

use base::discord::model::gateway::event::Event;

#[allow(clippy::module_name_repetitions)]
pub trait EventExt {
    fn as_str(&self) -> &str;
}

impl EventExt for Event {
    fn as_str(&self) -> &str {
        match self {
            Event::BanAdd(_) => "GUILD_BAN_ADD",
            Event::BanRemove(_) => "GUILD_BAN_REMOVE",
            Event::ChannelCreate(_) => "CHANNEL_CREATE",
            Event::ChannelDelete(_) => "CHANNEL_DELETE",
            Event::ChannelUpdate(_) => "CHANNEL_UPDATE",
            Event::ChannelPinsUpdate(_) => "CHANNEL_PINS_UPDATE",
            Event::GatewayHello(_) => "HELLO",
            Event::GatewayReconnect => "RECONNECT",
            Event::GuildCreate(_) => "GUILD_CREATE",
            Event::GuildDelete(_) => "GUILD_DELETE",
            Event::GuildEmojisUpdate(_) => "GUILD_EMOJIS_UPDATE",
            Event::GuildIntegrationsUpdate(_) => "GUILD_INTEGRATIONS_UPDATE",
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
            _ => "UNKNOWN_EVENT",
        }
    }
}
