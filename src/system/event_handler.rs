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
            GuildCreate,
            MemberUpdate,
            MessageCreate,
            Ready
        }
    }
};

use twilight_http::Client;

use twilight_mention::Mention;

use twilight_model::channel::message::AllowedMentions;

use crate::{
    command_system::{
        task_context::{
            MemberUpdateTaskContext,
            MemberUpdateTaskContextRef,
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
                BlockedMentionsDetectionTask,
                BlockedNicknameDetectionTask,
                BlockedWordsOrTokensDetectionTask,
                ConsecutiveCapitalLettersDetectionTask,
                DomainDetectionTask,
                InviteDetectionTask,
                ZalgoDetectionTask,
                ZalgoNicknameDetectionTask
            }
        }
    },
    state_enums::{
        censorship::{
            CensorshipProcess,
            NicknameCensorshipProcess
        }
    },
    system::{
        caching::SystemCache,
        state_machine::StateMachine,
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
                channel_id::ChannelId,
                role_id::RoleId
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
            ),
            "system::event_handler::EventHandler::ready"
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
                            guild_id),
                    "system::event_handler::EventHandler::guild_create"
                );

                if vector.contains(&guild_id) {
                    Logger::log_debug("Guild is whitelisted.",
                                      "system::event_handler::EventHandler::guild_create");
                }
                else {
                    Logger::log_debug("Guild is not whitelisted. Leaving guild.",
                                      "system::event_handler::EventHandler::guild_create");

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
        Logger::log_verbose(format!("Shard {} is connecting to the Discord gateway.", payload.shard_id),
                            "system::event_handler::EventHandler::shard_connecting");

        Ok(())
    }

    crate async fn shard_connected(payload: Connected) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is connected to the Discord gateway. The heartbeat interval is {} ms.",
                payload.shard_id,
                payload.heartbeat_interval
            ),
            "system::event_handler::EventHandler::shard_connected"
        );

        Ok(())
    }

    crate async fn shard_reconnecting(payload: Reconnecting) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is reconnecting to the Discord gateway.",
                payload.shard_id
            ),
            "system::event_handler::EventHandler::shard_reconnecting"
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
            ),
            "system::event_handler::EventHandler::shard_disconnected"
        );

        Ok(())
    }

    crate async fn shard_identifying(payload: Identifying) -> SystemResult<()> {
        Logger::log_verbose(
            format!(
                "Shard {} is identifying with the Discord gateway.",
                payload.shard_id,
            ),
            "system::event_handler::EventHandler::shard_identifying"
        );

        Ok(())
    }
    
    crate async fn message_create(payload: Box<MessageCreate>,
                                  http: Client,
                                  mut levelling_cache: SystemCache<String, bool>) -> SystemResult<()> {
        let config_string = http.clone().get_guild_configuration(payload.guild_id.unwrap()).await?;
        let config = quick_xml::de::from_str::<BotConfig>(&config_string)?;

        if let Some(ref plugins) = config.clone().plugins {
            if let Some(ref levelling_system) = plugins.levelling_system_plugin {
                let cooldown = if let Some(ref seconds_per_point) = levelling_system.seconds_per_point {
                    *seconds_per_point
                }
                else {
                    60
                };

                if !{
                    let member = http.guild_member(payload.guild_id.unwrap(), payload.author.id)
                        .await?.expect("Member is none.");

                    if let Some(ref included) = levelling_system.included_roles {
                        included.role_ids.is_empty() ||
                            member.roles.iter().any(|role_id|
                                included.role_ids.contains(&RoleId { id: role_id.into_inner_u64()} )
                            )
                    }
                    else {
                        false
                    }
                } {
                    return Ok(());
                }

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
                        .allowed_mentions(AllowedMentions::default())
                        .reply(payload.id)
                        .await?;
                }

                if xp > 0 {
                    levelling_cache.insert(
                        format!(
                            "guild_{}.user_{}", payload.guild_id.unwrap(), payload.author.id
                        ),
                        true,
                        Some(Duration::from_secs(cooldown))
                    );
                }
            }
        }

        let mut state_machine = StateMachine::new_with_state(CensorshipProcess::Initialized);
        let member = http.clone().guild_member(payload.guild_id.unwrap(), payload.author.id).await?.unwrap();
        let member_available_roles = member
            .roles
            .iter()
            .filter(|role_id| {
                if let Some(levels) = config.role_permission_levels.clone() {
                    levels.contains_key(&role_id.into_inner_u64())
                }
                else {
                    false
                }
            })
            .map(|role_id| role_id.into_inner_u64())
            .collect::<Vec<_>>();
        let mut roles_iter = member_available_roles.iter();

        if let Some(ref plugins) = config.plugins {
            if let Some(censorship) = plugins.censorship_plugin.clone() {
                for level in censorship.censorship_levels.levels {
                    if roles_iter.any(|id| {
                        if let Some(lvl) = config.role_permission_levels.clone().unwrap().get(id) {
                            *lvl >= level.level.level_integer as u32
                        }
                        else {
                            false
                        }
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
                                ),
                            ),
                            config.clone()
                        ).await?;

                        state_machine.update_state(CensorshipProcess::ZalgoFiltered);

                        if level.filter_invite_links == Some(true) {
                            InviteDetectionTask::execute_task(
                                TaskContext::MessageCreate(
                                    MessageCreateTaskContext(
                                        Arc::new(
                                            MessageCreateTaskContextRef::new(
                                                http.clone(),
                                                payload.author.clone(),
                                                payload.0.clone()
                                            )
                                        )
                                    ),
                                ),
                                config.clone()
                            ).await?;
                        }

                        state_machine.update_state(CensorshipProcess::InvitesFiltered);

                        if level.filter_domains == Some(true) {
                            DomainDetectionTask::execute_task(
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
                                ),
                                config.clone()
                            ).await?;
                        }

                        state_machine.update_state(CensorshipProcess::DomainsFiltered);

                        BlockedWordsOrTokensDetectionTask::execute_task(
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
                            ),
                            config.clone()
                        ).await?;

                        state_machine.update_state(CensorshipProcess::BlockedWordsOrTokensFiltered);

                        BlockedMentionsDetectionTask::execute_task(
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
                            ),
                            config.clone()
                        ).await?;

                        state_machine.update_state(CensorshipProcess::BlockedMentionsFiltered);

                        ConsecutiveCapitalLettersDetectionTask::execute_task(
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
                            ),
                            config.clone()
                        ).await?;

                        state_machine.update_state(CensorshipProcess::ConsecutiveCapitalLettersFiltered);

                        // TO BE IMPLEMEMTED

                        state_machine.update_state(CensorshipProcess::Completed);
                    }

                    if state_machine.state() == CensorshipProcess::Completed {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    crate async fn member_update(payload: Box<MemberUpdate>, http: Client) -> SystemResult<()> {
        let config_string = http.clone().get_guild_configuration(payload.guild_id).await?;
        let config = quick_xml::de::from_str::<BotConfig>(&config_string)?;
        let member = http.clone().guild_member(payload.guild_id, payload.user.id).await?.unwrap();
        let mut state_machine = StateMachine::new_with_state(NicknameCensorshipProcess::Initialized);

        ZalgoNicknameDetectionTask::execute_task(
            TaskContext::MemberUpdate(
                MemberUpdateTaskContext(
                    Arc::new(
                        MemberUpdateTaskContextRef::new(
                            http.clone(),
                            member.clone(),
                        )
                    )
                )
            ),
            config.clone()
        ).await?;

        state_machine.update_state(NicknameCensorshipProcess::ZalgoNicknamesFiltered);

        BlockedNicknameDetectionTask::execute_task(
            TaskContext::MemberUpdate(
                MemberUpdateTaskContext(
                    Arc::new(
                        MemberUpdateTaskContextRef::new(
                            http.clone(),
                            member,
                        )
                    )
                )
            ),
            config
        ).await?;

        state_machine.update_state(NicknameCensorshipProcess::BlockedNicknamesFiltered);

        state_machine.update_state(NicknameCensorshipProcess::Completed);

        Ok(())
    }

    // Section: Custom Events

    crate async fn command_executed(payload: Box<CommandExecuted>) -> SystemResult<()> {
        Logger::log_info(
            format!("Command '{}' is successfully executed in {}.",
                    payload.command, payload.guild_name),
            "system::event_handler::EventHandler::command_executed"
        );

        Ok(())
    }

    crate async fn command_failed(payload: Box<CommandFailed>) -> SystemResult<()> {
        Logger::log_error(
            format!("Command '{}' failed due to an error: '{}'.",
                    payload.command, payload.error),
            "system::event_handler::EventHandler::command_failed"
        );

        Ok(())
    }

    crate async fn command_received(_payload: Box<CommandReceived>) -> SystemResult<()> {
        Logger::log_verbose("Command received; identifying command.",
                            "system::event_handler::EventHandler::command_received");

        Ok(())
    }

    crate async fn command_identified(payload: String) -> SystemResult<()> {
        Logger::log_verbose(format!("Command identified: '{}'", payload),
                            "system::event_handler::EventHandler::command_identified");

        Ok(())
    }
}
