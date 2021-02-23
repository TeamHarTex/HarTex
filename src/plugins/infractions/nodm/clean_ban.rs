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

crate struct CleanBanCommand;

impl Command for CleanBanCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("nodmcleanban")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
                                            -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user_id = arguments.next().unwrap().to_string();

        let days = arguments.next().unwrap_or("0").to_string();

        let reason = arguments.into_remainder().unwrap().to_string();

        Box::pin(infractions_clean_ban_command(ctx, user_id, days, reason))
    }

    fn precommand_check<'asynchronous_trait, C>(ctx: CommandContext<'asynchronous_trait>,
                                                params: PrecommandCheckParameters, check: C)
                                                -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> where
        C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
            -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::resolve(check(ctx, params)))
    }
}

async fn infractions_clean_ban_command(ctx: CommandContext<'_>, user: String, delete_message_days: String,
                                       reason: String) -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;
    let guild_id = ctx.message.guild_id.unwrap();

    let user_id = if let Ok(user_id) = user.parse::<u64>() {
        UserId(user_id)
    }
    else if let Ok(user_id) = UserId::parse(user.as_str()) {
        user_id
    }
    else {
        ctx.http_client.clone().create_message(channel_id)
            .content("<:red_x:705623424675872859> The specified user ID or days of messages to delete is invalid.")?
            .allowed_mentions()
            .replied_user(false)
            .build()
            .reply(ctx.message.id)
            .await?;

        return Err(box CommandError("User ID cannot be none.".to_string()));
    };

    let infraction_id = format!("{:x}", Sha3_224::digest(
        format!("{}{}{}", guild_id, user_id, reason).as_str().as_bytes()));

    ctx.http_client.clone().create_ban(guild_id, user_id).delete_message_days(delete_message_days.parse().unwrap_or(0))?.await?;
    ctx.http_client.clone().add_user_infraction(infraction_id, guild_id, user_id, reason.clone(),
                                                InfractionType::Ban).await?;
    ctx.http_client.clone().create_message(channel_id)
        .content(format!(
            "<:green_check:705623382682632205> Successfully banned user with ID: `{}` for `{}`", user_id, reason))?
        .allowed_mentions()
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
