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
        precommand_checks::{
            SystemResult
        },
        Task,
        TaskContext
    },
    utilities::{
        url_detection::url_detected
    },
    xml_deserialization::{
        BotConfig
    }
};

crate struct DomainDetectionTask;

impl Task for DomainDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_domain_detection_task(ctx, config))
    }
}

async fn censorship_domain_detection_task(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        let message_parts = payload.message.content.split(" ").collect::<Vec<_>>();

        if message_parts.iter().any(|part| {
            url_detected(part.to_string()).is_some()
        }) {
            if let Some(ref plugins) = config.plugins {
                if let Some(ref censorship) = plugins.censorship_plugin {
                    for level in &censorship.censorship_levels.levels {
                        if level.filter_domains == Some(true) {
                            if let Some(whitelisteds) = level.whitelisted_domains.clone() {
                                if !whitelisteds.domains.iter().any(|domain| {
                                    payload.message.content.to_lowercase().contains(domain.to_lowercase())
                                }) {
                                    payload.http_client.clone().delete_message(payload.message.channel_id, payload.message.id).await?;
                                }
                            }
                            else if let Some(blacklisteds) = level.blacklisted_domains.clone() {
                                if blacklisteds.domains.iter().any(|domain| {
                                    payload.message.content.to_lowercase().contains(domain.to_lowercase())
                                }) {
                                    payload.http_client.clone().delete_message(payload.message.channel_id, payload.message.id).await?;
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
