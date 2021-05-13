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
        RoleId,
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
        GetGuildConfiguration
    },
    SystemResult
};

use crate::utilities::FutureResult;

use crate::xml_deserialization::BotConfig;

crate struct MuteCommand;

impl Command for MuteCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("mute")
    }

    fn aliases(&self) -> Vec<String> {
        vec![String::from("dmmute")]
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            mut arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        let to_parse = arguments.next().unwrap();
        let user_id = if let Ok(uid) = UserId::parse(to_parse) {
            Some(uid)
        }
        else if let Ok(uid) = to_parse.parse() {
            Some(UserId(uid))
        }
        else {
            None
        };

        let reason = arguments.into_remainder().unwrap().to_string();

        Box::pin(infractions_mute_command(ctx, user_id, reason))
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

async fn infractions_mute_command(ctx: CommandContext<'_>, user_id: Option<UserId>, reason: String)
    -> SystemResult<()> {
    let guild_id = ctx.message.guild_id.unwrap();
    let channel_id = ctx.message.channel_id;

    let guild_name = if let Ok(Some(guild)) = ctx.http_client.clone().guild(guild_id).await {
        guild.name
    }
    else {
        "unknown".to_string()
    };

    if let Some(uid) = user_id {
        let guild_config = ctx.http_client.clone().get_guild_configuration(guild_id).await?;
        let config = quick_xml::de::from_str::<BotConfig>(guild_config.as_str())?;

        let warning_id = format!("{:x}", Sha3_224::digest(
            format!("{}{}{}", guild_id, uid, reason).as_str().as_bytes()));

        if let Some(plugins) = config.plugins {
            return if let Some(infraction_plugin) = plugins.infractions_plugin {
                if let Some(mute_command) = infraction_plugin.mute_command {
                    if let Some(muted_role) = mute_command.muted_role {
                        let role_id = RoleId(muted_role.role_id);

                        if let Ok(Some(user)) = ctx.http_client.user(uid).await {
                            ctx.http_client.clone().add_user_infraction(warning_id.clone(),
                                                                        guild_id, uid, reason.clone(),
                                                                        InfractionType::Mute).await?;

                            ctx.http_client.clone().add_guild_member_role(guild_id, uid, role_id).await?;

                            if let Some(role_to_remove) = mute_command.role_to_remove {
                                ctx.http_client.clone().remove_guild_member_role(guild_id, uid,
                                                                                 RoleId(role_to_remove.role_id)).await?;
                            }

                            ctx.http_client.clone().create_message(ctx.message.channel_id)
                                .content(
                                    format!(
                                        "<:green_check:705623382682632205> Successfully muted user {} (ID: `{}`). Reason: `{}`. Infraction ID: `{}`",
                                        user.mention(), uid.0, reason, warning_id))?
                                .allowed_mentions(AllowedMentions::default()).reply(ctx.message.id).await?;

                            let dm_channel = ctx.http_client.clone().create_private_channel(uid).await?;

                            ctx.http_client.clone()
                                .create_message(dm_channel.id)
                                .content(format!("You have been muted in guild `{}` (ID: `{}`). Reason: `{}`",
                                                 guild_name, guild_id.0, reason))?
                                .await?;

                            Ok(())
                        }
                        else {
                            Err(box CommandError("User not found.".to_string()))
                        }
                    }
                    else {
                        Err(box CommandError("Muted role is not set.".to_string()))
                    }
                }
                else {
                    Err(box CommandError("Cannot find MuteCommand property in configuration.".to_string()))
                }
            }
            else {
                Err(box CommandError("Cannot find configuration for InfractionsPlugin.".to_string()))
            }
        }
        else {
            ctx.http_client
                .clone().create_message(channel_id)
                .content("<:red_x:705623424675872859> Muted role is not set.")?
                .allowed_mentions(AllowedMentions::default())
                .reply(ctx.message.id)
                .await?;

            Err(box CommandError("Muted role is not set.".to_string()))
        }
    }
    else {
        Err(box CommandError("User ID cannot be none.".to_string()))
    }
}
