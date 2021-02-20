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
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use twilight_model::{
    id::GuildId
};

use crate::{
    logging::logger::Logger,
    models::{
        levelling_system::Leaderboard
    },
    system::{
        twilight_http_client_extensions::{
            error::ClientExtensionResult,
            Pending
        },
    }
};

crate struct GetGuildLeaderboard {
    future: Option<Pending<Leaderboard>>,

    guild_id: GuildId
}

impl GetGuildLeaderboard {
    crate fn new(guild_id: GuildId) -> Self {
        Self {
            future: None,

            guild_id
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug("Attempting to create connection to HarTexBetaLevellingSystem database.");

        self.future.replace(Box::pin(
            request(self.guild_id)
        ));

        Ok(())
    }
}

impl Future for GetGuildLeaderboard {
    type Output = ClientExtensionResult<Leaderboard>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(future) = self.as_mut().future.as_mut() {
                return future.as_mut().poll(cx);
            }

            if let Err(error) = self.start() {
                return Poll::Ready(Err(error));
            }
        }
    }
}

unsafe impl Send for GetGuildLeaderboard {}

async fn request(guild_id: GuildId) -> ClientExtensionResult<Leaderboard> {
    todo!()
}
