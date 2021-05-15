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
    channel::message::AllowedMentions,
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

use crate::utilities::{
    duration::parse_duration,
    FutureResult
};

crate struct TempbanCommand;

impl Command for TempbanCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("tempban")
    }

    fn aliases(&self) -> Vec<String> {
        vec![String::from("dmtempban")]
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user = arguments.next().unwrap().to_string();
        let duration = arguments.next().unwrap_or("10s").to_string();
        let reason = arguments.into_remainder().unwrap_or("No reason specified").to_string();

        Box::pin(infractions_tempban_command(ctx, user, duration, reason))
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

async fn infractions_tempban_command(ctx: CommandContext<'_>, user: String, duration: String, reason: String) -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();

    let guild_name = if let Ok(Some(guild)) = ctx.http_client.clone().guild(guild_id).await {
        guild.name
    }
    else {
        "unknown".to_string()
    };

    let user_id = if let Ok(uid) = UserId::parse(user.as_str()) {
        uid
    }
    else if let Ok(uid) = user.parse() {
        UserId(uid)
    }
    else {
        return Err(box CommandError("The specified User ID is invalid.".to_string()));
    };

    let infraction_id = format!("{:x}", Sha3_224::digest(
        format!("{}{}{}", guild_id, user_id, reason.clone()).as_str().as_bytes()));

    let dur = if let Ok(duration) = parse_duration(duration.clone()) {
        duration
    }
    else {
        return Err(box CommandError(String::from("Invalid duration to parse.")))
    };

    ctx.http_client.clone()
        .add_user_infraction(infraction_id.clone(), guild_id, user_id, reason.clone(), InfractionType::Ban)
        .await?;

    ctx.http_client.clone().create_ban(guild_id, user_id).await?;

    ctx.http_client.clone().create_message(ctx.message.channel_id)
        .content(
            format!(
                "<:green_check:705623382682632205> Successfully temporarily banned user {} (ID: `{}`) for `{}`. Reason: `{}`. Infraction ID: `{}`",
                user_id.mention(), user_id.0, duration.clone(), reason, infraction_id))?
        .allowed_mentions(AllowedMentions::default())
        .reply(ctx.message.id).await?;

    let dm_channel = ctx.http_client.clone().create_private_channel(user_id).await?;
    ctx.http_client.clone()
        .create_message(dm_channel.id)
        .content(format!("You have been temporarily banned from guild `{}` for `{}` (ID: `{}`). Reason: `{}`",
                         guild_name, duration, guild_id.0, reason))?
        .await?;

    tokio::time::sleep(dur).await;

    ctx.http_client.clone().delete_ban(guild_id, user_id).await?;

    Ok(())
}
