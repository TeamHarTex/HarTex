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
    env::*,
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
    },
};

use crate::command_system::CommandError;
use crate::logging::logger::Logger;
use crate::system::{
    twilight_http_client_extensions::{
        error::ClientExtensionResult,
        Pending
    },
    twilight_id_extensions::IntoInnerU64
};

crate struct GetUserExperience {
    future: Option<Pending<(UserId, u64, u64)>>,

    guild_id: GuildId,
    user_id: UserId
}

impl GetUserExperience {
    crate fn new(guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            future: None,

            guild_id,
            user_id
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug("Attempting to create connection to HarTexBetaLevellingSystem database.".to_string(),
        "system::twilight_http_client_extensions::levelling_system::get_user_experience::GetUserExperience::start");

        self.future.replace(
            Box::pin(
                request(self.guild_id, self.user_id)
            )
        );

        Ok(())
    }
}

impl Future for GetUserExperience {
    type Output = ClientExtensionResult<(UserId, u64, u64)>;

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

unsafe impl Send for GetUserExperience {}

async fn request(guild_id: GuildId, user_id: UserId) -> ClientExtensionResult<(UserId, u64, u64)> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_LEVELLING_SYSTEM") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    let query_result = sqlx::query(
        &format!(
            // language=SQL
            "SELECT * FROM guild_{} WHERE user_id = $1",
            guild_id.into_inner_u64()
        )
    )
        .bind(user_id.into_inner_u64().to_string())
        .fetch_one(&connection)
        .await;

    match query_result {
        Ok(row) => {
            let level = row.get::<&str, &str>("lvl").parse::<u64>()?;
            let xp = row.get::<&str, &str>("xp").parse::<u64>()?;

            Ok((user_id, level, xp))
        },
        Err(error) => {
            Logger::log_error(format!("Could not get user level and experience due to an error: '{:?}'", error),
                              "system::twilight_http_client_extensions::levelling_system::get_user_experience::request");

            Err(box error)
        }
    }
}
