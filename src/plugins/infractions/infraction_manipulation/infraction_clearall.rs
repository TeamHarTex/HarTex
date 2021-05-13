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

use twilight_cache_inmemory::InMemoryCache;

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
    twilight_http_client_extensions::{
        ClearUserInfractions
    },
    SystemResult
};

use crate::utilities::FutureResult;

crate struct InfractionClearallCommand;

impl Command for InfractionClearallCommand {
    fn name(&self) -> String {
        String::from("inf")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("inf clear-all")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let string = arguments.next().unwrap_or("unknown");

        Box::pin(infractions_infraction_clearall_command(ctx, string.to_string()))
    }

    fn precommand_checks<'asynchronous_trait, C: 'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters, checks: Box<[C]>)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> + Send + Sync {
        Box::pin(
            async move {
                for check in checks.iter() {
                    if let Err(error) = check(ctx.clone(), params.clone()).await {
                        return Err(error);
                    } else {
                        continue;
                    }
                } Ok(())
            }
        )
    }
}

async fn infractions_infraction_clearall_command(ctx: CommandContext<'_>, user: String) -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();
    let user_id = if let Ok(id) = UserId::parse(&user) {
        id
    }
    else if let Ok(id) = user.parse() {
        UserId(id)
    }
    else {
        return Err(box CommandError("Specified User ID is invalid.".to_string()))
    };

    ctx.http_client.clone().clear_user_infractions(guild_id, user_id).await?;
    ctx.http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .allowed_mentions(AllowedMentions::default())
        .reply(ctx.message.id)
        .content("<:green_check:705623382682632205> Operation successful.")?
        .await?;

    Ok(())
}
