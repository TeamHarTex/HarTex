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
        twilight_id_extensions::IntoInnerU64,
        SystemResult,
    },
    xml_deserialization::{
        plugin_management::models::channel_id::ChannelId,
        BotConfig
    }
};

crate struct MessageCharLimitDetectionTask;

impl Task for MessageCharLimitDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_memory_character_limit_exceeded_detection_task(ctx, config))
    }
}

async fn censorship_memory_character_limit_exceeded_detection_task(ctx: TaskContext, config: BotConfig)
    -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship_plugin) = plugins.censorship_plugin {
                for level in &censorship_plugin.censorship_levels.levels {
                    if level.message_length_limit == 0 {
                        return Ok(());
                    }

                    if let Some(ref char_limit_channel_whitelist) = level.message_length_limit_channel_whitelist {
                        if char_limit_channel_whitelist.channel_ids.contains(
                            &ChannelId {
                                id: payload.message.channel_id.into_inner_u64()
                            }
                        ) {
                            return Ok(());
                        }

                        if payload.message.content.len() as u64 > level.message_length_limit {
                            payload.http_client.clone()
                                .delete_message(payload.message.channel_id, payload.message.id)
                                .await?;
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
