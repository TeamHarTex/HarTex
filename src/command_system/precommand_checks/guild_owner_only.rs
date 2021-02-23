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

use crate::{
    command_system::{
        CommandContext,
        CommandError,
        PrecommandCheckParameters
    },
    system::{
        SystemResult
    }
};

use super::PrecommandCheck;

crate struct GuildOwnerOnly;

impl PrecommandCheck for GuildOwnerOnly {
    fn execute_check<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(guild_owner_only(ctx, params))
    }
}

async fn guild_owner_only(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters)
    -> SystemResult<()> {
    if let (Some(guild_id), Some(cache)) = (ctx.message.guild_id, params.cache) {
        let guild = cache.guild(guild_id).unwrap();

        if ctx.author.id == guild.owner_id {
            Ok(())
        }
        else {
            Err(box CommandError("Command executor is not the guild owner.".to_string()))
        }
    }
    else {
        Err(box CommandError("Both the Guild ID and in-memory cache cannot be None.".to_string()))
    }
}
