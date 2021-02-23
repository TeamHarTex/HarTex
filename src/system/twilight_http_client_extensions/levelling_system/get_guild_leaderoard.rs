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
    env::*,
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use sqlx::{
    error::{
        Result as SqlxResult
    },
    postgres::{
        PgPool,
        PgRow
    },
    Row
};

use twilight_model::{
    id::{
        GuildId,
        UserId
    }
};

use crate::{
    command_system::CommandError,
    logging::logger::Logger,
    models::{
        levelling_system::{
            Leaderboard,
            LeaderboardEntry
        }
    },
    system::{
        twilight_http_client_extensions::{
            error::ClientExtensionResult,
            Pending
        },
        twilight_id_extensions::IntoInnerU64
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
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_LEVELLING_SYSTEM") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    let table_exists =  match sqlx::query(
        // language=SQL
        "SELECT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = $1)"
    )
        .bind(format!("guild_{}", guild_id.into_inner_u64()))
        .fetch_one(&connection)
        .await {
        Ok(row) => {
            let exists: bool = row.get("exists");

            exists
        },
        Err(error) => {
            Logger::log_error(format!("Failed to check if table exists. Error: {}", error.as_database_error().unwrap()));

            return Err(box error);
        }
    };

    if !table_exists {
        return Ok(Leaderboard::new_with_vector(Vec::new()));
    }

    match sqlx::query(
        &format!(
            // language=SQL
            "SELECT * FROM {}",
            format!("guild_{}", guild_id.into_inner_u64())
        )
    )
        .fetch_all(&connection)
        .await {
        Ok(rows) => {
            let mut entries = Vec::new();
            
            for row in rows {
                let user_id: String = row.get("user_id");
                let level: String = row.get("lvl");
                let experience: String = row.get("xp");
                
                entries.push(LeaderboardEntry::new(UserId(user_id.parse()?), level.parse()?, experience.parse()?))
            }

            Ok(Leaderboard::new_with_vector(entries))
        },
        Err(error) => {
            Logger::log_error(format!("Failed to retrieve leaderboard. Error: {}", error.as_database_error().unwrap()));

            Err(box error)
        }
    }
}
