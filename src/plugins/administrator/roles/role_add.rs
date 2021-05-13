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
    Mention
};

use twilight_model::{
    channel::message::AllowedMentions,
    id::{
        RoleId,
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

crate struct RoleAddCommand;

impl Command for RoleAddCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("role add")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user_id = arguments.next().unwrap_or("").to_string();
        let role_id = arguments.next().unwrap_or("").to_string();
        let reason = arguments.into_remainder().unwrap_or("No reason specified.").to_string();

        Box::pin(administrator_role_add_command(ctx, user_id, role_id, reason, cache))
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

async fn administrator_role_add_command(ctx: CommandContext<'_>, user_id: String, role_id: String,
                                       reason: String, cache: InMemoryCache) -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;
    let guild_id = ctx.message.guild_id.unwrap();

    if let Ok(uid) = user_id.parse() {
        if let Ok(rid) = role_id.parse() {
            let (actual_user_id, actual_role_id) = (UserId(uid), RoleId(rid));
            let role = if let Some(role) = cache.role(actual_role_id) {
                role
            }
            else {
                ctx.http_client.clone().create_message(channel_id)
                    .content(format!("<:red_x:705623424675872859> Role `{}` not found.", role_id))?
                    .allowed_mentions(AllowedMentions::default())
                    .reply(ctx.message.id)
                    .await?;

                return Err(box CommandError("Role not found.".to_string()));
            };

            ctx.http_client.clone().add_guild_member_role(guild_id, actual_user_id, actual_role_id).await?;
            ctx.http_client.clone().create_message(channel_id)
                .content(format!("<:green_check:705623382682632205> Added role {} to user `{}` for `{}`",
                                 role.mention(), uid, reason))?
                .allowed_mentions(AllowedMentions::default())
                .reply(ctx.message.id)
                .await?;

            Ok(())
        }
        else {
            ctx.http_client.clone().create_message(channel_id)
                .content(format!("<:red_x:705623424675872859> Could not find role `{}` in guild.", role_id))?
                .allowed_mentions(AllowedMentions::default())
                .reply(ctx.message.id)
                .await?;

            Err(box CommandError("Invalid role ID.".to_string()))
        }
    }
    else {
        ctx.http_client.clone().create_message(channel_id)
            .content(format!("<:red_x:705623424675872859> Could not find user `{}` in guild.", user_id))?
            .allowed_mentions(AllowedMentions::default())
            .reply(ctx.message.id)
            .await?;

        Err(box CommandError("Invalid user ID.".to_string()))
    }
}
