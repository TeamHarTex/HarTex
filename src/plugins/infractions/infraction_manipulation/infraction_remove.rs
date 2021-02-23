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
        RemoveUserInfraction
    },
    SystemResult
};

use crate::utilities::FutureResult;

crate struct InfractionRemoveCommand;

impl Command for InfractionRemoveCommand {
    fn name(&self) -> String {
        String::from("inf")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("inf remove")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user_id = match arguments.next() {
            Some(arg) => {
                if let Ok(id) = UserId::parse(arg) {
                    Some(id)
                }
                else if let Ok(int) = arg.parse() {
                    Some(UserId(int))
                }
                else {
                    None
                }
            },
            _ => None
        };

        let infraction_id = arguments.next().unwrap_or("");

        Box::pin(infractions_remove_command(ctx, user_id, infraction_id.to_string()))
    }

    fn precommand_check<'asynchronous_trait, C>(ctx: CommandContext<'asynchronous_trait>,
                                                params: PrecommandCheckParameters, check: C)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::resolve(Box::pin(check(ctx, params))))
    }
}

async fn infractions_remove_command(ctx: CommandContext<'_>, user_id: Option<UserId>, infraction_id: String)
    -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;

    if let (Some(gid), Some(uid)) = (ctx.message.guild_id, user_id) {
        ctx.http_client.clone().remove_user_infraction(gid, uid, infraction_id.clone()).await?;
        ctx.http_client.clone().create_message(channel_id).reply(ctx.message.id)
            .content(
                format!("<:green_check:705623382682632205> Infraction ID: `{}` is successfully removed.",
                        infraction_id
                )
            )?.allowed_mentions().replied_user(false).build().await?;

        Ok(())
    }
    else {
        Err(box CommandError("Guild ID and UserID cannot be null.".to_string()))
    }
}
