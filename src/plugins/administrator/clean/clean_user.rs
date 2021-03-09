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

use twilight_mention::{
    ParseMention
};

use twilight_model::{
    id::{
        MessageId,
        UserId
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

crate struct CleanUserCommand;

impl Command for CleanUserCommand {
    fn name(&self) -> String {
        String::from("clean")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("clean user")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user = arguments.next().unwrap_or("unknown").to_string();
        let number = arguments.next().unwrap_or("10").to_string();

        Box::pin(administrator_clean_user_command(ctx, user, number))
    }

    fn precommand_checks<'asynchronous_trait, C: 'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                                 params: PrecommandCheckParameters, checks: Box<[C]>)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> where
        C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
            -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> + Send + Sync {
        Box::pin(
            async move {
                for check in checks.iter() {
                    if let Err(error) = check(ctx.clone(), params.clone()).await {
                        return Err(error);
                    }
                    else {
                        continue;
                    }
                }

                Ok(())
            }
        )
    }
}

async fn administrator_clean_user_command(ctx: CommandContext<'_>, user: String, number_of_messages: String)
    -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;
    let user_id = if let Ok(u) = UserId::parse(&user) {
        u
    }
    else if let Ok(ru64) = user.parse() {
        UserId(ru64)
    }
    else {
        ctx.http_client.clone().create_message(channel_id)
            .content(format!("<:red_x:705623424675872859> Invalid user ID: `{}`", user))?.await?;

        return Err(box CommandError("Invalid user ID.".to_string()))
    };

    let number = if let Ok(int) = number_of_messages.parse::<u64>() {
        int
    }
    else {
        ctx.http_client.clone().create_message(channel_id)
            .content(format!("<:red_x:705623424675872859> Invalid limit: `{}`", number_of_messages))?.await?;

        return Err(box CommandError("Number of messages to delete is not a number.".to_string()));
    };

    let message_ids = ctx
        .http_client
        .clone()
        .channel_messages(channel_id)
        .limit(number)?
        .await?
        .iter()
        .filter(|message| message.author.id == user_id)
        .map(|message| message.id)
        .collect::<Vec<MessageId>>();

    ctx.http_client.clone().delete_messages(channel_id, message_ids.clone()).await?;
    ctx.http_client.clone().create_message(channel_id).content(
        format!(
            "<:green_check:705623382682632205> Deleted `{}` messages sent by user `{}` successfully!"
            , message_ids.len(), user_id))?.await?;

    Ok(())
}
