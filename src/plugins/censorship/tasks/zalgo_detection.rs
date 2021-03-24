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
    utilities::{
        zalgo_detection::zalgo_detected
    },
    xml_deserialization::BotConfig
};

crate struct ZalgoDetectionTask;

impl Task for ZalgoDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, _config: BotConfig) -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_zalgo_detection_task(ctx))
    }
}

async fn censorship_zalgo_detection_task(ctx: TaskContext) -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        let message = payload.message.clone();
        let zalgo_detected = zalgo_detected(&message.content);

        if zalgo_detected {
            payload.http_client.clone().delete_message(message.channel_id, message.id).await?;
        }

        Ok(())
    }
    else {
        unreachable!()
    }
}
