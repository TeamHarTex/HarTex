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

use twilight_embed_builder::{
    EmbedBuilder,
    EmbedFieldBuilder
};

use twilight_mention::{
    Mention
};

use twilight_model::{
    channel::message::AllowedMentions,
    id::{
        RoleId
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
    SystemResult,
};

use crate::utilities::FutureResult;

crate struct RoleinfoCommand;

impl Command for RoleinfoCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("role-info")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let role_id = arguments.next().unwrap_or("").to_string();

        Box::pin(administrator_roleinfo_command(ctx, role_id, cache))
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

async fn administrator_roleinfo_command(ctx: CommandContext<'_>, role_id: String, cache: InMemoryCache)
    -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;

    if let Ok(r_id) = role_id.parse() {
        let role_id_search = RoleId(r_id);

        if let Some(role) = cache.role(role_id_search) {
            let guild_id = ctx.message.guild_id.unwrap();
            let members_who_have_the_role =
                cache
                    .guild_members(guild_id)
                    .unwrap()
                    .iter()
                    .map(|id|
                        cache
                            .member(guild_id, *id)
                            .unwrap()
                    )
                    .filter(|member| member.roles.contains(&role_id_search))
                    .count();

            let embed = EmbedBuilder::new()
                .title(format!("Role ID: {}", r_id))
                .color(0x03_BE_FC)
                .field(EmbedFieldBuilder::new("Role Name", &role.name).inline())
                .field(EmbedFieldBuilder::new("Role ID", &role.id.0.to_string()).inline())
                .field(EmbedFieldBuilder::new("Role Colour", format!("0x{:X}", role.color)).inline())
                .field(EmbedFieldBuilder::new("Role Mention", format!("{}", role.mention())).inline())
                .field(EmbedFieldBuilder::new("Role Members", members_who_have_the_role.to_string()).inline())
                .field(EmbedFieldBuilder::new("Role Hoisted", role.hoist.to_string()).inline())
                .field(EmbedFieldBuilder::new("Role Position", role.position.to_string()).inline())
                .field(EmbedFieldBuilder::new("Role Mentionable", role.mentionable.to_string()).inline())
                .field(EmbedFieldBuilder::new("Role Managed", role.managed.to_string()).inline())
                .build()?;

            ctx.http_client.clone()
                .create_message(channel_id).embed(embed)?
                .allowed_mentions(AllowedMentions::default()).reply(ctx.message.id).await?;

            Ok(())
        }
        else {
            ctx.http_client.clone()
                .create_message(channel_id).content(
                    format!("<:red_x:705623424675872859> Could not find role with role id `{}` in guild.", r_id)
                )?
                .allowed_mentions(AllowedMentions::default()).reply(ctx.message.id).await?;

            Err(box CommandError("The role id points to an invalid role.".to_string()))
        }
    }
    else {
        ctx.http_client.clone()
            .create_message(channel_id).content(
                format!("<:red_x:705623424675872859> Could not find role with role id `{}` in guild.", role_id))?
            .allowed_mentions(AllowedMentions::default()).reply(ctx.message.id).await?;

        Err(box CommandError("The specified role_id is invalid.".to_string()))
    }
}
