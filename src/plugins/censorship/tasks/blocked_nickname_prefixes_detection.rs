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

use regex::{
    Regex
};

use crate::{
    command_system::{
        Task,
        TaskContext
    },
    system::{
        twilight_id_extensions::IntoInnerU64,
        SystemResult,
    },
    xml_deserialization::{
        plugin_management::{
            models::{
                censorship::{
                    prohibited_nickname_prefixes::{
                        ProhibitedNicknamePrefixes
                    }
                },
                channel_id::ChannelId
            }
        },
        BotConfig
    }
};

crate struct BlockedNicknamePrefixesDetection;

impl Task for BlockedNicknamePrefixesDetection {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_blocked_nickname_prefixes_detection_task(ctx, config))
    }
}

async fn censorship_blocked_nickname_prefixes_detection_task(ctx: TaskContext, config: BotConfig)
    -> SystemResult<()> {
    if let TaskContext::MemberUpdate(payload) = ctx {
        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship_plugin) = plugins.censorship_plugin {
                for level in &censorship_plugin.censorship_levels.levels {
                    if let Some(ref prohibited_prefixes) = level.prohibited_nickname_prefixes {
                        if prohibited_prefixes.prohibited_nickname_prefixes.clone()
                            .into_iter()
                            .any(|prefix| {
                                if let Some(nickname) = payload.member.nick.clone() {
                                    nickname.starts_with(&nickname)
                                }
                                else {
                                    false
                                }
                            }) {
                            payload.http_client
                                .update_guild_member(payload.member.guild_id, payload.member.user.id)
                                .nick(Some(if let Some(default_name) = level.zalgo_filtered_default_nickname.clone() {
                                    default_name
                                }
                                else {
                                    String::from("Censored Nickname")
                                }))?
                                .await;
                        }
                    }
                }
            }
        }
    }
    else {
        unreachable!()
    }

    Ok(())
}
