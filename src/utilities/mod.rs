///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

crate mod constants;
crate mod duration;
crate mod image_processing;
crate mod levelling_system;
crate mod zalgo_detection;

use std::{
    error::Error,
    future::Future,
    pin::Pin
};

use crate::{
    system::{
        SystemResult
    }
};

crate struct FutureResult;

impl FutureResult {
    pub async fn ok() -> SystemResult<()> {
        Ok(())
    }

    pub async fn err(error: Box<dyn Error + Send + Sync>) -> SystemResult<()> {
        Err(error)
    }

    pub async fn resolve<'asynchronous_trait>(
        result: Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>>)
        -> SystemResult<()> {
        result.await
    }
}
