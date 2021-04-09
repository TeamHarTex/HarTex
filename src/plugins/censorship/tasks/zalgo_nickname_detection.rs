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
    Sha3_224,
    Digest
};

use crate::{
    command_system::{
        Task,
        TaskContext
    },
    system::{
        model::infractions::InfractionType,
        twilight_http_client_extensions::AddUserInfraction,
        SystemResult,
    },
    utilities::{
        zalgo_detection::zalgo_detected
    },
    xml_deserialization::BotConfig
};

crate struct ZalgoNicknameDetectionTask;

impl Task for ZalgoNicknameDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_zalgo_nickname_detection(ctx, config))
    }
}

async fn censorship_zalgo_nickname_detection(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MemberUpdate(payload) = ctx {
        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship_plugin) = plugins.censorship_plugin {
                for level in &censorship_plugin.censorship_levels.levels {
                    if level.filter_zalgo_nicknames == Some(true) {
                        if let Some(nickname) = payload.member.nick.clone() {
                            if zalgo_detected(&nickname) {
                                payload.http_client
                                    .update_guild_member(payload.member.guild_id, payload.member.user.id)
                                    .nick(Some(if let Some(default_name) = level.zalgo_filtered_default_nickname.clone() {
                                        default_name
                                    }
                                    else {
                                        String::from("Censored Nickname")
                                    }))?
                                    .await;

                                if level.warn_on_censored == Some(true) {
                                    let warning_id = format!(
                                        "{:x}",
                                        Sha3_224::digest(
                                            format!(
                                                "{}{}{}",
                                                payload.member.guild_id.0,
                                                payload.member.user.id.0,
                                                String::from("Auto Moderation: Blocked mention censored.")
                                            ).as_bytes()
                                        )
                                    );

                                    payload.http_client.clone()
                                        .add_user_infraction(warning_id,
                                                             payload.member.guild_id,
                                                             payload.member.user.id,
                                                             String::from("Auto Moderation: Blocked mention censored."),
                                                             InfractionType::Warning).await?;
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
