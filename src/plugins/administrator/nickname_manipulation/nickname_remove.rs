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
    channel::message::AllowedMentions,
    id::UserId
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

crate struct NicknameRemoveCommand;

impl Command for NicknameRemoveCommand {
    fn name(&self) -> String {
        String::from("nickname")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("nickname change")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
                                            -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user = arguments.into_remainder().unwrap_or("unknown");

        Box::pin(administrator_nickname_remove_command(ctx, user.to_string()))
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

async fn administrator_nickname_remove_command(ctx: CommandContext<'_>, user: String) -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;
    let user_id = if let Ok(id) = UserId::parse(&user) {
        id
    }
    else if let Ok(int) = user.parse() {
        UserId(int)
    }
    else {
        ctx.http_client.clone()
            .create_message(channel_id)
            .content(format!("<:red_x:705623424675872859> Invalid user ID encountered: `{}`.", user))?
            .await?;

        return Err(box CommandError("Invalid user ID.".to_string()))
    };

    let user = if let Some(u) = ctx.http_client.clone().user(user_id).await? {
        u
    }
    else {
        ctx.http_client.clone()
            .create_message(channel_id)
            .content(format!("<:red_x:705623424675872859> User `{}` not found.", user))?
            .await?;

        return Err(box CommandError("User not found.".to_string()))
    };

    ctx.http_client
        .clone()
        .update_guild_member(ctx.message.guild_id.unwrap(), user_id)
        .await?;
    ctx.http_client.clone()
        .create_message(channel_id)
        .content(
            format!(
                "<:red_x:705623424675872859> Successfully removed nickname for user `{}#{}`.",
                user.name,
                user.discriminator
            )
        )?
        .allowed_mentions(AllowedMentions::default())
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
