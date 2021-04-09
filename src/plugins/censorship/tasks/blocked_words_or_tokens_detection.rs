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
        precommand_checks::{
            SystemResult
        },
        Task,
        TaskContext
    },
    system::{
        model::infractions::InfractionType,
        twilight_http_client_extensions::AddUserInfraction,
        twilight_id_extensions::IntoInnerU64
    },
    utilities::{
        blocked_words_or_tokens_detection::blocked_words_or_tokens_detected
    },
    xml_deserialization::{
        plugin_management::{
            models::channel_id::ChannelId
        },
        BotConfig
    }
};

crate struct BlockedWordsOrTokensDetectionTask;

impl Task for BlockedWordsOrTokensDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_blocked_words_or_tokens_detection_task(ctx, config))
    }
}

async fn censorship_blocked_words_or_tokens_detection_task(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        let message = payload.message.content.clone();

        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship) = plugins.censorship_plugin {
                for level in &censorship.censorship_levels.levels {
                    if let Some(whitelist) = level.clone().blocked_words_or_tokens_channel_whitelist {
                        if whitelist.channel_ids.contains(&ChannelId { id: payload.message.channel_id.into_inner_u64() }) {
                            return Ok(());
                        }
                    }

                    let mut blocked = if let Some(words) = level.clone().prohibited_words {
                        words.prohibited_word
                    }
                    else {
                        vec![]
                    };

                    blocked.append(&mut {
                        if let Some(tokens) = level.clone().prohibited_tokens {
                            tokens.prohibited_token
                        }
                        else {
                            vec![]
                        }
                    });

                    if blocked_words_or_tokens_detected(message.clone(), Some(blocked)) {
                        payload.http_client.delete_message(
                            payload.message.channel_id,
                            payload.message.id
                        ).await?;

                        if level.warn_on_censored == Some(true) {
                            let warning_id = format!(
                                "{:x}",
                                Sha3_224::digest(
                                    format!(
                                        "{}{}{}",
                                        payload.message.guild_id.unwrap().0,
                                        payload.author.id.0,
                                        String::from("Auto Moderation: Blocked mention censored.")
                                    ).as_bytes()
                                )
                            );

                            payload.http_client.clone()
                                .add_user_infraction(warning_id,
                                                     payload.message.guild_id.unwrap(),
                                                     payload.message.author.id,
                                                     String::from("Auto Moderation: Blocked mention censored."),
                                                     InfractionType::Warning).await?;
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
