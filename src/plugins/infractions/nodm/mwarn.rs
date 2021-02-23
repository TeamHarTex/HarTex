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

use sha3::{
    Digest,
    Sha3_224
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_mention::{
    Mention,
    parse::ParseMention,
};

use twilight_model::{
    id::{
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
    model::{
        infractions::InfractionType
    },
    twilight_http_client_extensions::{
        AddUserInfraction
    },
    SystemResult
};

use crate::utilities::FutureResult;

crate struct MwarnCommand;

impl Command for MwarnCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("nodmmwarn")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let mut users = Vec::<String>::new();

        while let Some(string) = arguments.next() {
            match string {
                "-r" | "--reason" => break,
                _ => users.push(string.to_string())
            }
        }

        let reason = arguments.into_remainder().unwrap_or("No reason specified.");

        Box::pin(infractions_mwarn_command(ctx, users, reason.to_string()))
    }

    fn precommand_check<'asynchronous_trait, C>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters, check: C)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::resolve(check(ctx, params)))
    }
}

async fn infractions_mwarn_command(ctx: CommandContext<'_>, users: Vec<String>, reason: String) -> SystemResult<()> {
    let mut users_to_warn = Vec::new();
    let guild_id = ctx.message.guild_id.unwrap();
    let channel_id = ctx.message.channel_id;

    let guild_name = if let Ok(Some(guild)) = ctx.http_client.clone().guild(guild_id).await {
        guild.name
    }
    else {
        "unknown".to_string()
    };

    for user in users {
        if let Ok(user_id) = UserId::parse(&user) {
            users_to_warn.push(user_id);
        }
        else if let Ok(user_id) = user.parse() {
            users_to_warn.push(UserId(user_id));
        }
        else {
            return Err(box CommandError("Specified User ID is invalid.".to_string()));
        }
    };

    for user in users_to_warn {
        let warning_id = format!("{:x}", Sha3_224::digest(
            format!("{}{}{}", guild_id.0, user.0, reason.clone()).as_bytes()));

        if ctx.author.id != user {
            if let Ok(Some(user_)) = ctx.http_client.user(user).await {
                ctx.http_client.clone().add_user_infraction(
                    warning_id.clone(), ctx.message.guild_id.unwrap(), user,
                    reason.clone(), InfractionType::Warning).await?;

                ctx.http_client.clone().create_message(channel_id).content(
                    format!(
                        "<:green_check:705623382682632205> Successfully warned user {} (ID: `{}`). Reason: `{}`. Infraction ID: `{}`"
                        , user_.mention(), user.0, reason.clone(), warning_id.clone()))?
                    .reply(ctx.message.id).allowed_mentions().replied_user(false).build().await?;

                let dm_channel = ctx.http_client.clone().create_private_channel(user).await?;

                ctx.http_client.clone().create_message(dm_channel.id).content(
                    format!(
                        "You received a warning in guild {} (ID: `{}`). Reason: `{}`",
                        guild_name, guild_id.0, reason.clone()
                    ))?.await?;
            }
        }
        else {
            return Err(box CommandError("Cannot give a warning to the command executor himself/herself.".to_string()))
        }
    };

    Ok(())
}
