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

use rand::{
    Rng,
    thread_rng
};

use twilight_cache_inmemory::{
    InMemoryCache
};

use twilight_model::channel::message::AllowedMentions;

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext
};

use crate::system::{
    SystemResult,
};

crate struct RandintCommand;

impl Command for RandintCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("randint")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let start = arguments.next().unwrap_or("1").parse().unwrap_or(1);
        let end = arguments.next().unwrap_or("10").parse().unwrap_or(10);

        Box::pin(utilities_randint_command(ctx, start, end))
    }
}

async fn utilities_randint_command(ctx: CommandContext<'_>, range_start: u128, range_end: u128) -> SystemResult<()> {
    let allowed_mentions = AllowedMentions::default();
    let message = ctx
        .http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .content("Generating random number; please wait...")?
        .allowed_mentions(allowed_mentions)
        .reply(ctx.message.id)
        .await?;

    let generated_number = thread_rng().gen_range(range_start..=range_end);

    ctx.http_client
        .clone()
        .update_message(ctx.message.channel_id, message.id)
        .content(format!("The random number from {} to {} is: `{}`", range_start, range_end, generated_number))?
        .allowed_mentions(allowed_mentions.clone()).await?;

    Ok(())
}
