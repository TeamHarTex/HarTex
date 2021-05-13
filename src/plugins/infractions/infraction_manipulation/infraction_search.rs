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

use pad::PadStr;

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
        GetLocalUserInfractions
    },
    SystemResult
};

use crate::utilities::FutureResult;

crate struct InfractionSearchCommand;

impl Command for InfractionSearchCommand {
    fn name(&self) -> String {
        String::from("inf")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("inf search")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let query = arguments.next().unwrap_or("").to_string();

        Box::pin(infractions_infraction_search_command(ctx, query))
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

async fn infractions_infraction_search_command(ctx: CommandContext<'_>, query: String)
    -> SystemResult<()> {
    let user_id = if let Ok(uid) = UserId::parse(query.as_str()) {
        uid
    }
    else if let Ok(uid) = query.parse() {
            UserId(uid)
    }
    else {
        return Err(box CommandError("Querying infractions with not a user id is not currently supported.".to_string()));
    };

    let infractions = ctx.http_client.clone().get_local_user_infractions(
        ctx.message.guild_id.unwrap(), user_id).await?;

    if infractions.is_empty() {
        ctx.http_client.clone().create_message(ctx.message.channel_id)
            .content("The user has no infractions.")?.allowed_mentions(AllowedMentions::default())
            .reply(ctx.message.id).await?;
    }
    else {
        let mut message: String =
            "```ID                                                       | Type     | Reason\n".to_owned()
                + "-------------------------------------------------------- | -------- | ----" +
                "-------------------------------------------------------------------";

        infractions.iter().for_each(|inf| {
            message.push_str(format!("\n{} | {} | {}", inf.infraction_id, inf.infraction_type.to_string()
                .as_str().pad_to_width(8), inf.reason).as_str())
        });

        message.push_str("\n```");

        ctx.http_client.clone().create_message(ctx.message.channel_id).content(message)?.allowed_mentions(AllowedMentions::default())
            .reply(ctx.message.id).await?;
    }

    Ok(())
}
