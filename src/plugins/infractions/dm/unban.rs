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

crate struct UnbanCommand;

impl Command for UnbanCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("unban")
    }

    fn aliases(&self) -> Vec<String> {
        vec![String::from("dmunban")]
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
                                            -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        let to_parse = arguments.next().unwrap();
        let user_id = if let Ok(uid) = UserId::parse(to_parse) {
            Some(uid)
        }
        else if let Ok(uid) = to_parse.parse() {
            Some(UserId(uid))
        }
        else {
            None
        };

        let reason = arguments.into_remainder().unwrap().to_string();

        Box::pin(infractions_unban_command(ctx, user_id, reason))
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

async fn infractions_unban_command(ctx: CommandContext<'_>, user_id: Option<UserId>, reason: String) -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;
    let guild_id = ctx.message.guild_id.unwrap();

    let guild_name = if let Ok(Some(guild)) = ctx.http_client.clone().guild(guild_id).await {
        guild.name
    }
    else {
        "unknown".to_string()
    };

    if let Some(uid) = user_id {
        let infraction_id = format!("{:x}", Sha3_224::digest(
            format!("{}{}{}", guild_id, uid, reason).as_str().as_bytes()));

        let dm_channel = ctx.http_client.clone().create_private_channel(uid).await?.id;

        ctx.http_client.clone().add_user_infraction(infraction_id, guild_id, uid, reason.clone(),
                                                    InfractionType::Unban).await?;

        ctx.http_client.clone()
            .create_message(dm_channel)
            .content(
                format!("You have been unbanned from guild `{}` (ID: `{}`). Reason: `{}`",
                        guild_name, guild_id.0, reason.clone()))?.await?;

        ctx.http_client.clone().delete_ban(guild_id, uid).await?;

        ctx.http_client.clone().create_message(channel_id)
            .content(format!(
                "<:green_check:705623382682632205> Successfully unbanned user with ID: `{}` for `{}`", uid, reason))?
            .allowed_mentions()
            .replied_user(false)
            .build()
            .reply(ctx.message.id)
            .await?;

        Ok(())
    }
    else {
        ctx.http_client.clone().create_message(channel_id)
            .content("<:red_x:705623424675872859> The specified user ID is invalid.")?
            .allowed_mentions()
            .replied_user(false)
            .build()
            .reply(ctx.message.id)
            .await?;

        Err(box CommandError("User ID cannot be none.".to_string()))
    }
}
