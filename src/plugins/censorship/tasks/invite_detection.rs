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
        twilight_http_client_extensions::GetGuildConfiguration,
        twilight_id_extensions::IntoInnerU64,
        SystemResult,
    },
    utilities::{
        invite_detection::invite_detected
    },
    xml_deserialization::BotConfig
};

crate struct InviteDetectionTask;

impl Task for InviteDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_invite_detection_task(ctx, config))
    }
}

async fn censorship_invite_detection_task(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        let message = payload.message.clone();

        if let (true, Some(invite)) = invite_detected(message.content) {
            if let Some(ref plugin) = config.plugins {
                if let Some(ref censorship) = plugin.censorship_plugin {
                    for level in &censorship.levels {
                        if level.filter_invite_links == Some(true) {
                            if let Some(whitelisted) = level.clone().whitelisted_guild_invites {
                                let twilight_invite = payload.http_client.clone().invite(invite.clone().code).await?;

                                if let Some(actual_invite) = twilight_invite {
                                    if let Some(invite_guild) = actual_invite.guild {
                                        if !whitelisted.whitelists.iter()
                                            .any(|whitelist|
                                                whitelist.id == Some(invite_guild.id.into_inner_u64())
                                            ) {
                                            payload.http_client.clone()
                                                .delete_message(payload.message.channel_id, payload.message.id)
                                                .await?;
                                        }
                                    }
                                }
                            }
                        }
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
