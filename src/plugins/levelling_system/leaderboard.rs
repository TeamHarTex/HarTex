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
    fs,
    pin::Pin,
    str::FromStr
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_mention::ParseMention;

use twilight_model::{
    id::UserId
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    CommandError
};

use crate::std_extensions::{
    FormatAsIec8000013PrefixPostfixDecimalMultiplerString
};

use crate::system::{
    twilight_http_client_extensions::{
        GetGuildLeaderboard,
        GetUserExperience
    },
    SystemResult
};

crate struct LeaderboardCommand;

impl Command for LeaderboardCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("leaderboard")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, arguments: Arguments<'asynchronous_trait>, cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        todo!()
    }
}

async fn levelling_system_leaderboard_command(ctx: CommandContext<'_>) -> SystemResult<()> {
    todo!()
}
