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
    InMemoryCache
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    CommandError
};
use crate::command_system::precommand_checks::SystemResult;

crate struct InterestingCommand;

impl Command for InterestingCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("interesting")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send>> {
        Box::pin(utilities_interesting_command(ctx))
    }
}

async fn utilities_interesting_command(ctx: CommandContext<'_>) -> SystemResult<()> {
    ctx.http_client.clone().create_message(ctx.message.channel_id)
        .content("https://cdn.discordapp.com/attachments/752463311500607539/821698873277939742/image0.png")?
        .allowed_mentions()
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .await?;
    
    Ok(())
}
