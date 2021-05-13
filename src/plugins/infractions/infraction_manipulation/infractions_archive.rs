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
    fs::File,
    pin::Pin
};

use chrono::{
    Local
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_model::channel::message::AllowedMentions;

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    PrecommandCheckParameters
};

use crate::system::{
    twilight_http_client_extensions::GetGuildInfractions,
    twilight_id_extensions::IntoInnerU64,
    SystemResult
};

use crate::utilities::{
    FutureResult
};

crate struct InfractionsArchiveCommand;

impl Command for InfractionsArchiveCommand {
    fn name(&self) -> String {
        String::from("inf")
    }

    fn fully_qualified_name(&self) -> String {
        String::from("inf archive")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(infractions_infractions_archive_command(ctx))
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

async fn infractions_infractions_archive_command(ctx: CommandContext<'_>) -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();
    let now = Local::now().timestamp();
    let csv_file =
        File::create(
            format!(
                "csv/{}_guildinfs_{}.csv",
                now,
                guild_id.into_inner_u64()
            )
        )?;
    let mut writer = csv::Writer::from_writer(csv_file);
    let infraction_map = ctx
        .http_client
        .clone()
        .get_guild_infractions(ctx.message.guild_id.unwrap())
        .await?;

    writer.write_record(&["User ID", "Infraction ID", "Infraction Type", "Reason"])?;

    for (user_id, infractions) in infraction_map {
        for infraction in infractions {
            writer.write_record(&[
                &format!("{}", user_id.into_inner_u64()),
                &infraction.infraction_id,
                &format!("{}", infraction.infraction_type),
                &infraction.reason
            ])?;
        }
    }

    writer.flush()?;

    ctx.http_client
        .clone()
        .create_message(ctx.message.channel_id)
        .allowed_mentions(AllowedMentions::default())
        .attachment(
            format!("{}_guildinfs_{}.csv", now, guild_id),
            std::fs::read(format!(
                "csv/{}_guildinfs_{}.csv",
                now,
                guild_id.into_inner_u64()
            ))?
        )
        .reply(ctx.message.id)
        .await?;

    std::fs::remove_file(format!(
        "csv/{}_guildinfs_{}.csv",
        now,
        guild_id.into_inner_u64()
    ))?;

    Ok(())
}
