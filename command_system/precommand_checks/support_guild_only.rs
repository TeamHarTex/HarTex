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

use std::{
    future::Future,
    pin::Pin
};

use twilight_model::{
    id::GuildId
};

use crate::{
    command_system::{
        CommandContext,
        CommandError,
        PrecommandCheckParameters
    },
    system::{
        SystemResult
    }
};

use super::PrecommandCheck;

crate struct SupportGuildOnly;

impl PrecommandCheck for SupportGuildOnly {
    fn execute_check<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _params: PrecommandCheckParameters)
        -> Pin<Box<dyn Future<Output = SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(support_guild_only(ctx))
    }
}

async fn support_guild_only(ctx: CommandContext<'asynchronous_trait>) -> SystemResult<()> {
    return if ctx.message.guild_id == Some(GuildId(631100984176672768)) {
        Ok(())
    }
    else {
        Err(box CommandError("The command can only be executed in the support guild.".to_string()))
    }
}

