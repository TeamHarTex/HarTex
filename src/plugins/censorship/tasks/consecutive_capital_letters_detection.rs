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

                    // We remove all spaces from the string to count consecutive capital letters,
                    // whilst completely ignoring spaces.
                    message.retain(|character| !character.is_whitespace());


                }
            }
        }
    }
    else {
        unreachable!()
    }

    Ok(())
}
