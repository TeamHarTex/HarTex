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
    pin::Pin,
};

use futures_util::future;

use twilight_cache_inmemory::InMemoryCache;

use twilight_model::{
    id::{
        GuildId,
    }
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
    twilight_http_client_extensions::{
        InitializeWhitelistedGuild
    },
    SystemResult
};

use crate::utilities::{
    FutureResult
};

crate struct SetupCommand;

impl Command for SetupCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("setup")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            _arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let guild_id = ctx.message.guild_id.unwrap();

        Box::pin(guild_owneronly_setup_command(ctx, guild_id))
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

async fn guild_owneronly_setup_command(ctx: CommandContext<'_>, guild_id: GuildId)
    -> SystemResult<()> {
    ctx.http_client.clone().initialize_whitelisted_guild(guild_id).await?;
    ctx.http_client.clone()
        .create_message(ctx.message.channel_id)
        .content("<:green_check:705623382682632205> Setup complete!")?
        .allowed_mentions()
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
