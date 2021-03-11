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
    time::Duration
};

use sha3::{
    Digest,
    Sha3_224
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_mention::{
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
        AddUserInfraction,
    },
    SystemResult
};

use crate::utilities::FutureResult;

crate struct MkickCommand;

impl Command for MkickCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("mkick")
    }

    fn aliases(&self) -> Vec<String> {
        vec![String::from("dmmkick")]
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let mut users = Vec::<String>::new();

        while let Some(string) = arguments.next() {
            match string {
                "-r" | "--reason" => break,
                _ => users.push(string.to_string())
            }
        }

        let reason = arguments.into_remainder().unwrap_or("No reason specified.");

        Box::pin(infractions_mkick_command(ctx, users, reason.to_string()))
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

async fn infractions_mkick_command(ctx: CommandContext<'_>, users: Vec<String>, reason: String) -> SystemResult<()> {
    let mut users_to_kick = Vec::new();
    let guild_id = ctx.message.guild_id.unwrap();

    let guild_name = if let Ok(Some(guild)) = ctx.http_client.clone().guild(guild_id).await {
        guild.name
    }
    else {
        "unknown".to_string()
    };

    for user in users {
        if let Ok(user_id) = UserId::parse(&user) {
            users_to_kick.push(user_id);
        }
        else if let Ok(user_id) = user.parse() {
            users_to_kick.push(UserId(user_id));
        }
        else {
            return Err(box CommandError("Specified User ID is invalid.".to_string()))
        }
    }

    for user_to_kick in users_to_kick {
        let infraction_id = format!("{:x}", Sha3_224::digest(
            format!("{}{}{}", guild_id.0, user_to_kick.0, reason.clone()).as_bytes()));

        let dm_channel = ctx.http_client.clone().create_private_channel(user_to_kick).await?.id;

        ctx.http_client.clone().add_user_infraction(infraction_id, guild_id, user_to_kick, reason.clone(),
                                                    InfractionType::Kick).await?;
        ctx.http_client.clone()
            .create_message(dm_channel)
            .content(
                format!("You have been kicked from guild `{}` (ID: `{}`). Reason: `{}`",
                        guild_name, guild_id.0, reason.clone()))?.await?;
        ctx.http_client.remove_guild_member(guild_id, user_to_kick);
        ctx.http_client.clone()
            .create_message(ctx.message.channel_id)
            .content(
                format!("<:green_check:705623382682632205> Successfully kicked user with ID: `{}` for `{}`",
                        user_to_kick, reason.clone()))?
            .allowed_mentions().replied_user(false).build()
            .reply(ctx.message.id)
            .await?;

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    Ok(())
}
