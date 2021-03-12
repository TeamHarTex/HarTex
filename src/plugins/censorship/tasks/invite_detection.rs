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
        invite_detection::invite_detected
    }
};

crate struct InviteDetectionTask;

impl Task for InviteDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send>> {
        Box::pin(censorship_invite_detection_task(ctx))
    }
}

async fn censorship_invite_detection_task(ctx: TaskContext) -> SystemResult<()> {
    todo!()
}
