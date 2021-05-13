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

use compound_duration::format_dhms;

use twilight_cache_inmemory::{
    InMemoryCache
};

use twilight_embed_builder::{
    EmbedBuilder,
    EmbedFieldBuilder
};

use twilight_model::channel::message::AllowedMentions;

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
};

use crate::system::{
    SystemResult,
};

crate struct UptimeCommand;

impl Command for UptimeCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("uptime")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            _arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(general_uptime_command(ctx))
    }
}

async fn general_uptime_command(ctx: CommandContext<'_>) -> SystemResult<()> {
    let elapsed = ctx.stopwatch.elapsed_seconds();
    let embed = EmbedBuilder::new()
        .title("Bot Uptime")
        .color(0x03_BE_FC)
        .field(EmbedFieldBuilder::new("Current Uptime", format_dhms(elapsed)))
        .build()?;

    ctx.http_client.create_message(ctx.message.channel_id).reply(ctx.message.id).embed(embed)?.allowed_mentions(AllowedMentions::default())
        .await?;

    Ok(())
}
