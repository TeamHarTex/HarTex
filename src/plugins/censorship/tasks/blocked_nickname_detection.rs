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

use crate::{
    command_system::{
        Task,
        TaskContext
    },
    system::{
        SystemResult,
    },
    xml_deserialization::BotConfig
};

crate struct BlockedNicknameDetectionTask;

impl Task for BlockedNicknameDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_blocked_nickname_detection_task(ctx, config))
    }
}

async fn censorship_blocked_nickname_detection_task(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MemberUpdate(payload) = ctx {
        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship_plugin) = plugins.censorship_plugin {
                for level in &censorship_plugin.censorship_levels.levels {
                    if level.filter_zalgo_nicknames == Some(true) {
                        if let Some(nickname) = payload.member.nick.clone() {
                            if let Some(ref list) = level.blocked_nicknames {
                                if list.blocked_nicknames.contains(&nickname) {
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

                        break;
                    }
                }
            }
        }

        Ok(())
    }
    else {
        unreachable!()
    }
}
