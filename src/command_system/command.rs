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

use super::{
    parser::{
        Arguments
    },
    CommandContext,
    PrecommandCheckParameters
};

use crate::system::SystemResult;

use crate::utilities::FutureResult;

use twilight_cache_inmemory::InMemoryCache;

crate trait Command {
    fn name(&self) -> String {
        String::from("default")
    }

    fn fully_qualified_name(&self) -> String;

    fn aliases(&self) -> Vec<String> {
        Vec::new()
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>>;

    fn precommand_check<'asynchronous_trait, C>(_ctx: CommandContext<'asynchronous_trait>,
                                                _params: PrecommandCheckParameters, _check: C)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::ok())
    }

    #[allow(clippy::boxed_local)]
    fn precommand_checks<'asynchronous_trait, C: 'asynchronous_trait>(_ctx: CommandContext<'asynchronous_trait>,
                                                 _params: PrecommandCheckParameters, _checks: Box<[C]>)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> + Send + Sync {
        Box::pin(FutureResult::ok())
    }

    #[allow(clippy::boxed_local)]
    fn any_precommand_check<'asynchronous_trait, C>(_ctx: CommandContext<'asynchronous_trait>,
                                                    _params: PrecommandCheckParameters, _check: Box<[C]>)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>>
        where
            C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(FutureResult::ok())
    }
}
