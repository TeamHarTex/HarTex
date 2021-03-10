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
    pin::Pin,
    sync::Arc
};

use twilight_cache_inmemory::{
    model::CachedMember,
    InMemoryCache,
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    PrecommandCheckParameters
};

use crate::system::{
    SystemResult,
};

use crate::utilities::FutureResult;

crate struct NorolesListCommand;

impl Command for NorolesListCommand {
    fn name(&self) -> String {
        String::from("noroles")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("noroles list")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send+ 'asynchronous_trait>> {
        Box::pin(administrator_noroles_list_command(ctx, cache))
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

async fn administrator_noroles_list_command(ctx: CommandContext<'_>, cache: InMemoryCache) -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();
    let members =
        cache
        .guild_members(guild_id)
        .unwrap()
        .iter()
        .map(|user_id| cache.member(guild_id, *user_id).unwrap())
        .filter(|member| (*member).roles.is_empty()).collect::<Vec<Arc<CachedMember>>>();
    let mut message = String::from("**__Members Without Roles__**\n");

    members.iter().for_each(|member| {
        message.push_str(&format!("{}#{}", member.user.name, member.user.discriminator));
    });

    ctx.http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .content(message)?
        .allowed_mentions()
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
