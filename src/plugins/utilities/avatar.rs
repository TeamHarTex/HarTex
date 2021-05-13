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
    InMemoryCache
};

use twilight_mention::ParseMention;

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
    CommandError
};

use crate::content_distribution_network::ContentDistributionNetwork;

use crate::system::{
    SystemResult,
};

crate struct AvatarCommand;

impl Command for AvatarCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("avatar")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let user = arguments.next().unwrap_or("default");

        Box::pin(utilities_avatar_command(ctx, user.to_string()))
    }
}

async fn utilities_avatar_command(ctx: CommandContext<'_>, user: String) -> SystemResult<()> {
    let id = if let Ok(user) = UserId::parse(&user) {
        user
    }
    else if let Ok(user) = user.parse() {
        UserId(user)
    }
    else {
        return Err(box CommandError("Invalid user ID.".to_string()))
    };

    let mut message = String::new();

    if let Some(user) = ctx.http_client.clone().user(id).await? {
        message.push_str(&if let Some(avatar) = user.avatar {
            ContentDistributionNetwork::user_avatar(id, avatar.clone(), avatar.starts_with("a_"))?
        }
        else {
            ContentDistributionNetwork::default_user_avatar(user.discriminator.parse()?)?
        })
    }
    else {
        message.push_str("<:red_x:705623424675872859> Cannot find user.")
    }

    ctx.http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .content(message)?
        .allowed_mentions(AllowedMentions::default())
        .reply(ctx.message.id)
        .await?;

    Ok(())
}
