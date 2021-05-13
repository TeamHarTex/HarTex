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
use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    CommandError,
    PrecommandCheckParameters
};

use twilight_model::channel::message::AllowedMentions;

use crate::system::{
    SystemResult,
};

use crate::utilities::FutureResult;

crate struct InvitesCommand;

impl Command for InvitesCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("invites")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(administrator_invites_command(ctx, cache))
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

async fn administrator_invites_command(ctx: CommandContext<'_>, cache: InMemoryCache) -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();
    let guild = cache.guild(guild_id).unwrap();
    let invites = ctx.http_client.clone().guild_invites(guild_id).await?;

    let mut message = format!("**__Invites of Guild {}__**\n", guild.name);

    invites.iter().for_each(|invite| {
        message.push_str(&format!("{}\n", invite.code));
    });

    ctx.http_client.clone().create_message(ctx.message.channel_id)
        .content(message)?
        .allowed_mentions(AllowedMentions::default())
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
