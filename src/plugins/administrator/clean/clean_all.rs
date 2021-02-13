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

use twilight_cache_inmemory::{
    InMemoryCache,
};

use twilight_model::{
    id::{
        MessageId
    }
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

crate struct CleanAllCommand;

impl Command for CleanAllCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("clean all")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let number_of_messages = arguments.next().unwrap_or("25");

        Box::pin(administrator_clean_all_command(ctx, number_of_messages.to_string()))
    }

    fn precommand_check<'asynchronous_trait, C>(ctx: CommandContext<'asynchronous_trait>,
                                                params: PrecommandCheckParameters, check: C)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::resolve(check(ctx, params)))
    }
}

async fn administrator_clean_all_command(ctx: CommandContext<'_>, number_of_messages: String)
    -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;

    if let Ok(limit) = number_of_messages.parse() {
        let channel_messages =
            ctx.http_client.clone().channel_messages(channel_id).limit(limit)?.await?.iter()
                .map(|message| message.id).collect::<Vec<MessageId>>();

        ctx.http_client.clone().delete_messages(channel_id, channel_messages).await?;

        ctx.http_client.clone().create_message(channel_id)
            .content(format!("<:green_check:705623382682632205> Deleted `{}` messages successfully!",
                             limit))?.await?;

        Ok(())
    }
    else {
        ctx.http_client.clone().create_message(channel_id)
            .content(format!("<:red_x:705623424675872859> Invalid limit: `{}`", number_of_messages))?.await?;

        Err(box CommandError("Specified limit cannot be parsed into a u64".to_string()))
    }
}
