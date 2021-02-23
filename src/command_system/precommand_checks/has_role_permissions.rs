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
    pin::Pin,
    sync::Arc
};

use twilight_model::{
    guild::Role
};

use crate::{
    command_system::{
        CommandContext,
        CommandError,
        PrecommandCheckParameters
    },
    system::{
        twilight_http_client_extensions::{
            GetGuildConfiguration
        },
        SystemResult
    },
    xml_deserialization::{
        BotConfig
    }
};

use super::PrecommandCheck;

crate struct HasRolePermissions;

impl PrecommandCheck for HasRolePermissions {
    fn execute_check<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(has_role_permissions(ctx, params))
    }
}

async fn has_role_permissions(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters)
    -> SystemResult<()> {
    if let Some(gid) = ctx.message.guild_id {
        let config_str = ctx.http_client.clone().get_guild_configuration(gid).await?;

        let config = quick_xml::de::from_str::<BotConfig>(&config_str)?;

        if let Some(cache) = params.cache {
            let mut roles = ctx.http_client
                .clone()
                .guild_member(gid, ctx.author.id)
                .await?
                .unwrap()
                .roles
                .iter()
                .map(|&role| {
                    cache.role(role).unwrap()
                })
                .collect::<Vec<Arc<Role>>>();

            roles.sort_by(|previous, now| {
                now.position.cmp(&previous.position)
            });

            if let Some(role_permission_levels) = config.role_permission_levels {
                if role_permission_levels.contains_key(&roles.first().unwrap().id.0) {
                    if let Some(minimum_permission_level) = params.minimum_permission_level {
                        let set_level = role_permission_levels.get(&roles.first().unwrap().id.0).unwrap();

                        if minimum_permission_level <= *set_level {
                            Ok(())
                        }
                        else {
                            Err(box CommandError("The user does not have the minimum required permission to execute this command.".to_string()))
                        }
                    }
                    else {
                        Err(box CommandError("Minimum permission level required cannot be none, as required by the check.".to_string()))
                    }
                }
                else {
                    Err(box CommandError("The user does not have the minimum required permission to execute this command".to_string()))
                }
            }
            else {
                Err(box CommandError("Permission levels are not set.".to_string()))
            }
        }
        else {
            Err(box CommandError("Cache cannot be none, as required by the check.".to_string()))
        }
    }
    else {
        Err(box CommandError("Guild ID cannot be none.".to_string()))
    }
}
