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
    convert::TryInto,
    future::Future,
    iter::IntoIterator,
    pin::Pin
};

use itertools::{
    Itertools
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

crate struct ConsecutiveCapitalLettersDetectionTask;

impl Task for ConsecutiveCapitalLettersDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_consecutive_capital_letters_detection_task(ctx, config))
    }
}

async fn censorship_consecutive_capital_letters_detection_task(ctx: TaskContext, config: BotConfig)
    -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        if let Some(ref plugins) = config.plugins {
            if let Some(ref censorship_plugin) = plugins.censorship_plugin {
                for level in &censorship_plugin.censorship_levels.levels {
                    if level.filter_capital_letters != Some(true) {
                        return Ok(());
                    }

                    let minimum_caps = if let Some(minimum) = level.minimum_consecutive_capital_letters {
                        minimum
                    }
                    else {
                        5
                    };

                    let mut message = payload.message.content.clone();
                    let is_uppercase = char::is_uppercase;

                    // We remove all spaces from the string to count consecutive capital letters,
                    // whilst completely ignoring spaces.
                    message.retain(|character| !character.is_whitespace());

                    let groups = message.char_indices().group_by(|&(_idx, c)| is_uppercase(c));

                    let longest_capital_substring = groups
                        .into_iter()
                        .filter_map(|(is_capital, mut group)| {
                            is_capital.then(|| {
                                let first = group.next().expect("group can't be empty");
                                let (end_idx, end_chr) = group.last().unwrap_or(first);
                                let (start_idx, _) = first;

                                &message[start_idx..end_idx + end_chr.len_utf8()]
                            })
                        })
                        .max_by_key(|s| s.len());

                    if longest_capital_substring.is_none() {
                        return Ok(());
                    }

                    // It is ok to .unwrap() here because we already know that the variable is not none.
                    if longest_capital_substring.unwrap().len() as u64 >= minimum_caps {
                        payload.http_client.clone()
                            .delete_message(payload.message.channel_id, payload.message.id)
                            .await?;
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
