///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

use std::{
    future::Future,
    pin::Pin
};

mod bot_owner_only;
mod guild_is_already_setup;
mod guild_text_channel_only;
mod guild_owner_only;
mod has_role_permissions;
mod support_guild_only;

crate use bot_owner_only::BotOwnerOnly;
crate use guild_is_already_setup::GuildIsAlreadySetup;
crate use guild_text_channel_only::GuildTextChannelOnly;
crate use guild_owner_only::GuildOwnerOnly;
crate use has_role_permissions::HasRolePermissions;
crate use support_guild_only::SupportGuildOnly;

crate use crate::{
    command_system::{
        CommandContext,
        PrecommandCheckParameters
    },
    system::{
        SystemResult
    }
};

crate trait PrecommandCheck {
    fn execute_check<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>>;
}
