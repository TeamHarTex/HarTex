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
    model::infraction_update_type::InfractionUpdateType,
    twilight_http_client_extensions::{
        UpdateUserInfraction
    },
    SystemResult
};

use crate::utilities::FutureResult;

crate struct InfractionReasonCommand;

impl Command for InfractionReasonCommand {
    fn name(&self) -> String {
        String::from("inf")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("inf reason")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user_id = arguments.next().unwrap_or("");
        let infraction_id = arguments.next().unwrap_or("unknown infraction");
        let new_reason = arguments.into_remainder().unwrap_or("no reason specified.");

        Box::pin(infractions_infraction_reason_command(ctx, user_id.to_string(), infraction_id.to_string(), new_reason.to_string()))
    }

    fn precommand_check<'asynchronous_trait, C>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters, check: C)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::resolve(check(ctx, params)))
    }
}

async fn infractions_infraction_reason_command(ctx: CommandContext<'_>, user_id: String, infraction_id: String, new_reason: String) -> SystemResult<()> {
    let user_id = if let Ok(id) = UserId::parse(&user_id) {
        id
    }
    else if let Ok(id) = user_id.parse() {
        UserId(id)
    }
    else {
        return Err(box CommandError("Specified User ID is invalid.".to_string()))
    };

    ctx.http_client
        .clone()
        .update_user_infraction(infraction_id, ctx.message.guild_id.unwrap(), user_id, InfractionUpdateType::Reason { new_reason })
        .await?;
    ctx.http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .allowed_mentions()
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .content("<:green_check:705623382682632205> Operation successful.")?
        .await?;

    Ok(())
}
