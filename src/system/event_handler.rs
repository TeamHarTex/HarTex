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
    sync::Arc
};

use tokio::{
    time::Duration
};

use twilight_model::{
    gateway::{
        event::{
            shard::{
                Connected,
                Connecting,
                Disconnected,
                Identifying,
                Reconnecting
            }
        },
        payload::{
            MessageCreate,
            GuildCreate,
            Ready
        }
    }
};

use twilight_http::Client;

use twilight_mention::Mention;

use crate::{
    command_system::{
        task_context::{
            MessageCreateTaskContext,
            MessageCreateTaskContextRef
        },
        Task,
        TaskContext,
    },
    logging::logger::Logger,
    plugins::{
        censorship::{
            tasks::{
                ZalgoDetectionTask
            }
        }
    },
    system::{
        caching::SystemCache,
        model::payload::{
            CommandExecuted,
            CommandFailed,
            CommandReceived,
        },
        twilight_http_client_extensions::{
            AddUserExperience,
            GetGuildConfiguration,
            GetWhitelistedGuilds
        },
        twilight_id_extensions::IntoInnerU64,
        Stopwatch,
        SystemResult
    },
    xml_deserialization::{
        plugin_management::{
            models::{
                channel_id::ChannelId
            }
        },
        BotConfig
    }
};

crate struct EventHandler;

impl EventHandler {
    // Section: Twilight Event

    crate async fn ready(payload: Box<Ready>, stopwatch: Stopwatch) -> SystemResult<()> {
        let current_user = payload.user;

        Logger::log_info(
            format!("{}#{} [ID: {}] has successfully startup, using Discord API v{}. Startup time is {} ms.",
                    current_user.name,
                    current_user.discriminator,
                    current_user.id.into_inner_u64(),
                    payload.version,
                    stopwatch.elapsed_milliseconds()
            )
        );

        Ok(())
    }

    crate async fn guild_create(payload: Box<GuildCreate>, http: Client) -> SystemResult<()> {
        let guild_id = payload.id;
        let guild = http.guild(guild_id).await?;

        match http.clone().get_whitelisted_guilds().await {
            Ok(vector) => {
                Logger::log_debug(
                    format!("Joined a new guild with ID {}. Checking whether the guild is whitelisted.",
                            guild_id)
                );

                if vector.contains(&guild_id) {
                    Logger::log_debug("Guild is whitelisted.");
                }
                else {
                    Logger::log_debug("Guild is not whitelisted. Leaving guild.");

                    if let Some(g) =  guild {
                        let owner_id = g.owner_id;

                        if let Some(u) = http.user(owner_id).await? {
                            let dm_channel = http.create_private_channel(u.id).await?;
                            let message_content = "Thank you for checking out HarTex and inviting it to your guild!\n\n".to_owned()
                                + "Unfortunately, it looks like your guild is not whitelisted. You may"
                                + "apply if your guild meets the following criteria:\n\n- Have at least 100"
                                + "members;\n- Always abide by the Discord Terms of Service and Community"
                                + "Guidelines;\n- Shall not have any NSFW channels; and\n- One member of "
                                + "your staff team shall stay in the support server for contacting"
                                + "purposes.\n\nServer Invite: discord.gg/s8qjxZK\n\nPlease go to our"
                                + "support server and run `hb.apply` to apply for a whitelist. \n\nWish you"
                                + "best of luck!";

                            http.create_message(dm_channel.id).content(message_content)?.await?;
                        }
                    }

                    http.leave_guild(guild_id).await?;
                }
            },
            Err(_error) => ()
        };

        // Update Current User nickname if a different one is set other than the default.
        let config_string = http.clone().get_guild_configuration(guild_id).await?;
        let guild_config = quick_xml::de::from_str::<BotConfig>(&config_string)?;
        let current_user = http.clone().current_user().await?.id;

        if let Some(customization) = guild_config.bot_customization {
            http.clone()
                .update_guild_member(guild_id, current_user)
                .nick(Some(customization.guild_nickname))?
                .await?;
        }

        Ok(())
    }

    crate async fn shard_connecting(payload: Connecting) -> SystemResult<()> {
        Logger::log_verbose(format!("Shard {} is connecting to the Discord gateway.", payload.shard_id));

        Ok(())
    }

    crate async fn shard_connected(payload: Connected) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is connected to the Discord gateway. The heartbeat interval is {} ms.",
                payload.shard_id,
                payload.heartbeat_interval
            )
        );

        Ok(())
    }

    crate async fn shard_reconnecting(payload: Reconnecting) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is reconnecting to the Discord gateway.",
                payload.shard_id
            )
        );

        Ok(())
    }

    crate async fn shard_disconnected(payload: Disconnected) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is disconnected from the Discord gateway with reason `{:?}` and close code `{:?}`.",
                payload.shard_id,
                payload.reason,
                payload.code
            )
        );

        Ok(())
    }

    crate async fn shard_identifying(payload: Identifying) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is identifying with the Discord gateway.",
                payload.shard_id,
            )
        );

        Ok(())
    }
    
    crate async fn message_create(payload: Box<MessageCreate>,
                                  http: Client,
                                  mut levelling_cache: SystemCache<String, bool>) -> SystemResult<()> {
        let xp = if let Some(_) = levelling_cache.get(&format!("guild_{}.user_{}", payload.guild_id.unwrap(), payload.author.id)) {
            0u64
        }
        else {
            crate::utilities::levelling_system::random_experience(15, 25)
        };

        let (level_up, level) = http.clone().add_user_experience(payload.guild_id.unwrap(), payload.author.id, xp).await?;

        if level_up {
            http.clone()
                .create_message(payload.channel_id)
                .content(
                    format!(
                        "Congratulations, {}! You reached **Level {}**!",
                        payload.author.mention(),
                        level
                    )
                )?
                .allowed_mentions()
                .replied_user(false)
                .build()
                .reply(payload.id)
                .await?;
        }

        if xp > 0 {
            levelling_cache.insert(
                format!(
                    "guild_{}.user_{}", payload.guild_id.unwrap(), payload.author.id
                ),
                true,
                Some(Duration::from_secs(60))
            );
        }

        let config_string = http.clone().get_guild_configuration(payload.guild_id.unwrap()).await?;
        let config = quick_xml::de::from_str::<BotConfig>(&config_string)?;

        if let Some(plugins) = config.plugins {
            if let Some(censorship) = plugins.censorship_plugin {
                for level in censorship
                    .levels {
                    if let Some(whitelist) = level.zalgo_channel_whitelist.clone() {
                        if !whitelist.channel_ids.contains(&ChannelId {
                            id: payload.channel_id.into_inner_u64()
                        }) {
                            ZalgoDetectionTask::execute_task(
                                TaskContext::MessageCreate(
                                    MessageCreateTaskContext(
                                        Arc::new(
                                            MessageCreateTaskContextRef::new(
                                                http.clone(),
                                                payload.author.clone(),
                                                payload.0.clone()
                                            )
                                        )
                                    )
                                )
                            ).await?;

                            // We already completed the task (when it can be executed); so we can just break out of the loop.
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // Section: Custom Events

    crate async fn command_executed(payload: Box<CommandExecuted>) -> SystemResult<()> {
        Logger::log_info(
            format!("Command '{}' is successfully executed in {}.",
                    payload.command, payload.guild_name)
        );

        Ok(())
    }

    crate async fn command_failed(payload: Box<CommandFailed>) -> SystemResult<()> {
        Logger::log_error(
            format!("Command '{}' failed due to an error: '{}'.",
                    payload.command, payload.error)
        );

        Ok(())
    }

    crate async fn command_received(_payload: Box<CommandReceived>) -> SystemResult<()> {
        Logger::log_verbose("Command received; identifying command.");

        Ok(())
    }

    crate async fn command_identified(payload: String) -> SystemResult<()> {
        Logger::log_verbose(format!("Command identified: '{}'", payload));

        Ok(())
    }
}
