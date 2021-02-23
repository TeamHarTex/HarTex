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

use twilight_model::{
    id::{
        GuildId
    }
};

use crate::{
    command_system::{
        CommandContext,
        CommandError,
        PrecommandCheckParameters
    },
    system::{
        twilight_http_client_extensions::{
            GetGuildConfiguration
        },
        SystemResult
    }
};

use super::PrecommandCheck;

crate struct GuildIsAlreadySetup;

impl PrecommandCheck for GuildIsAlreadySetup {
    fn execute_check<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, params: PrecommandCheckParameters)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(guild_is_already_setup(ctx, params.guild_id.unwrap()))
    }
}

async fn guild_is_already_setup(ctx: CommandContext<'asynchronous_trait>, guild_id: GuildId) -> SystemResult<()> {
    if ctx.http_client.clone().get_guild_configuration(guild_id).await.is_ok() {
        Err(box CommandError("Guild is already setup.".to_string()))
    }
    else {
        Ok(())
    }
}
