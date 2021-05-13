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

use rand::{
    random
};

use twilight_cache_inmemory::{
    InMemoryCache
};

use twilight_model::channel::message::AllowedMentions;

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext
};

use crate::system::{
    SystemResult,
};

crate struct CoinflipCommand;

impl Command for CoinflipCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("coinflip")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(utilities_coinflip_command(ctx))
    }
}

async fn utilities_coinflip_command(ctx: CommandContext<'_>) -> SystemResult<()> {
    let message = ctx
        .http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .content("Flipping a coin; please wait...")?
        .allowed_mentions(AllowedMentions::default())
        .replied_user(false)
        .build()
        .reply(ctx.message.id)
        .await?;
    let allowed_mentions = AllowedMentions::default();

    let result = random::<bool>();

    ctx.http_client
        .clone()
        .update_message(ctx.message.channel_id, message.id)
        .content(format!("The coin landed on **{}**.", if result { "head" } else { "tail" }))?
        .allowed_mentions(allowed_mentions).await?;

    Ok(())
}
