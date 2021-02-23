//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

use std::{
    future::Future,
    pin::Pin
};

use twilight_cache_inmemory::{
    InMemoryCache,
};

use twilight_http::{
    request::{
        channel::reaction::RequestReactionType
    }
};

use twilight_model::{
    guild::{
        Permissions
    },
    id::{
        EmojiId,
        RoleId
    },
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    CommandError,
    PrecommandCheckParameters
};

use crate::system::{
    SystemResult,
};

use crate::utilities::FutureResult;

crate struct VoicemuteDisableCommand;

impl Command for VoicemuteDisableCommand {
    fn name(&self) -> String {
        String::from("voicemute")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("voicmute disable")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(administrator_voicemute_disable_command(ctx, cache))
    }

    fn precommand_check<'asynchronous_trait, C>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters, check: C)
                                                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::resolve(check(ctx, params)))
    }
}

async fn administrator_voicemute_disable_command(ctx: CommandContext<'_>, cache: InMemoryCache) -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();
    let channel_id = ctx.message.channel_id;
    let voice_state = cache.voice_state(ctx.author.id, guild_id).unwrap();
    let allow = Permissions::USE_VAD;
    let deny = Permissions::VIEW_CHANNEL;

    let voice_channel = if let Some(ch) = voice_state.channel_id {
        ch
    }
    else {
        return Err(box CommandError("User not in voice channel".to_string()))
    };

    ctx.http_client
        .clone()
        .update_channel_permission(voice_channel, allow, deny)
        .role(RoleId(guild_id.0))
        .await?;
    ctx.http_client.clone().create_reaction(channel_id, ctx.message.id, RequestReactionType::Custom {
        id: EmojiId(705623382682632205),
        name: None
    });

    Ok(())
}
