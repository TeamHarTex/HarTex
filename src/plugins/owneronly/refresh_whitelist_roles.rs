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

use twilight_cache_inmemory::InMemoryCache;

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
    PrecommandCheckParameters
};

use crate::system::{
    twilight_http_client_extensions::{
        GetGuildConfiguration,
        GetWhitelistedGuilds
    },
    SystemResult
};

use crate::utilities::{
    constants::{
        hartex_guild_owner,
        verified_hartex_user
    }
};

use crate::xml_deserialization::{
    BotConfig
};

crate struct RefreshWhitelistRolesCommand;

impl Command for RefreshWhitelistRolesCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("refresh-whitelist-roles")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            _arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(owneronly_refresh_whitelist_roles_command(ctx, cache))
    }

    fn precommand_checks<'asynchronous_trait, C: 'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                                 params: PrecommandCheckParameters, checks: Box<[C]>)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>
        where
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

async fn owneronly_refresh_whitelist_roles_command(ctx: CommandContext<'_>, cache: InMemoryCache)
    -> SystemResult<()> {
    let whitelisted_guilds = ctx.http_client.clone().get_whitelisted_guilds().await?;
    let current_guild_id = ctx.message.guild_id.unwrap();
    let current_guild_users = cache.guild_members(current_guild_id).unwrap();

    for guild_id in whitelisted_guilds {
        let guild_config_str = ctx.http_client.clone().get_guild_configuration(guild_id).await?;
        let config = quick_xml::de::from_str::<BotConfig>(guild_config_str.as_str())?;
        let owner = cache.guild(guild_id).expect("Guild not found (barely happen!)").owner_id;

        // FIXME: Fetch audit log to detect owner change.
        if current_guild_users.contains(&owner) {
            ctx.http_client.clone().add_guild_member_role(current_guild_id, owner, hartex_guild_owner())
                .await?;
        }
        else {
            ctx.http_client.clone().remove_guild_member_role(current_guild_id, owner, hartex_guild_owner())
                .await?;
        }

        for user in config.dashboard.users {
            if current_guild_users.contains(&UserId(user.id)) {
                ctx.http_client.clone().add_guild_member_role(current_guild_id, UserId::from(user.id),
                                                              verified_hartex_user()).await?;
            }
            else {
                ctx.http_client.clone().remove_guild_member_role(current_guild_id, UserId::from(user.id),
                                                                 verified_hartex_user()).await?;
            }
        }
    }

    Ok(())
}
