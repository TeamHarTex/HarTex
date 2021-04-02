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
                    blocked_mentions::{
                        BlockedMention
                    }
                },
                channel_id::ChannelId
            }
        },
        BotConfig
    }
};

crate struct BlockedMentionsDetectionTask;

impl Task for BlockedMentionsDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_blocked_mentions_detection_task(ctx, config))
    }
}

async fn censorship_blocked_mentions_detection_task(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        let regex = Regex::new(r"<((@)|(#)|(@&))([0-9]+)>").unwrap();
        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship_plugin) = plugins.censorship_plugin {
                for level in &censorship_plugin.censorship_levels.levels {
                    if let Some(ref blocked_mentions) = level.prohibited_mentions {
                        if let Some(captures) = regex.captures(&payload.message.content) {
                            if let Some(whitelisted_channels) = level.clone().blocked_mentions_channel_whitelist {
                                if whitelisted_channels.channel_ids.contains(&ChannelId { id: payload.message.channel_id.into_inner_u64() }) {
                                    return Ok(());
                                }
                            }

                            for blocked in &blocked_mentions.blocked_mentions {
                                match blocked {
                                    BlockedMention::ChannelId(value) => {
                                        if let Some(r#match) = captures.get(1) {
                                            if r#match.as_str() != "#" {
                                                return Ok(());
                                            }

                                            if let Some(r#match) = captures.get(5) {
                                                if value != &r#match.as_str().parse::<u64>().unwrap() {
                                                    return Ok(());
                                                }

                                                payload.http_client.clone()
                                                    .delete_message(payload.message.channel_id, payload.message.id)
                                                    .await?;
                                            }
                                        }
                                    },
                                    BlockedMention::RoleId(value) => {
                                        if let Some(r#match) = captures.get(1) {
                                            if r#match.as_str() != "@&" {
                                                return Ok(());
                                            }

                                            if let Some(r#match) = captures.get(5) {
                                                if value != &r#match.as_str().parse::<u64>().unwrap() {
                                                    return Ok(());
                                                }

                                                payload.http_client.clone()
                                                    .delete_message(payload.message.channel_id, payload.message.id)
                                                    .await?;
                                            }
                                        }
                                    },
                                    BlockedMention::UserId(value) => {
                                        if let Some(r#match) = captures.get(1) {
                                            if r#match.as_str() != "@" {
                                                return Ok(());
                                            }

                                            if let Some(r#match) = captures.get(5) {
                                                if value != &r#match.as_str().parse::<u64>().unwrap() {
                                                    return Ok(());
                                                }

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
        }

        Ok(())
    }
    else {
        unreachable!()
    }
}
