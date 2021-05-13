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

use twilight_cache_inmemory::{
    InMemoryCache,
};

use twilight_mention::{
    ParseMention
};

use twilight_model::{
    channel::message::AllowedMentions,
    id::ChannelId
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
    SystemResult,
};

use crate::utilities::{
    FutureResult
};

crate struct SlowmodeDisableChannelCommand;

impl Command for SlowmodeDisableChannelCommand {
    fn name(&self) -> String {
        String::from("slowmode")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("slowmode disable channel")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
                                            -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let channel_id = arguments.into_remainder().unwrap_or("unknown").to_string();

        Box::pin(administrator_slowmode_disable_channel_command(ctx, channel_id))
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

async fn administrator_slowmode_disable_channel_command(ctx: CommandContext<'_>, channel: String) -> SystemResult<()> {
    let channel_id = if let Ok(id) = ChannelId::parse(&channel) {
        id
    }
    else if let Ok(id) = channel.parse() {
        ChannelId(id)
    }
    else {
        return Err(box CommandError("Invalid channel id.".to_string()))
    };

    ctx.http_client.clone().update_channel(channel_id).rate_limit_per_user(0)?.await?;
    ctx.http_client
        .clone()
        .create_message(channel_id)
        .allowed_mentions(AllowedMentions::default())
        .content(
            format!("<:green_check:705623382682632205> Disabled slowmode for channel <#{}>", channel_id.0)
        )?
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
