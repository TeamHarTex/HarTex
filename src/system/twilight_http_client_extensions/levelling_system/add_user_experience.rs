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

crate struct AddUserExperience {
    future: Option<Pending<(bool, u64)>>,

    guild_id: GuildId,
    user_id: UserId,
    experience: u64
}

impl AddUserExperience {
    crate fn new(guild_id: GuildId, user_id: UserId, experience: u64) -> Self {
        Self {
            future: None,

            guild_id,
            user_id,
            experience
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug("Attempting to create connection to HarTexBetaLevellingSystem database.".to_string(),
        "system::twilight_http_extensions::levelling_system::add_user_experience::AddUserExperience::start");

        self.future.replace(Box::pin(
            request(
                self.guild_id, self.user_id, self.experience
            )
        ));

        Ok(())
    }
}

impl Future for AddUserExperience {
    type Output = ClientExtensionResult<(bool, u64)>;

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

unsafe impl Send for AddUserExperience {}

async fn request(guild_id: GuildId, user_id: UserId, experience: u64) -> ClientExtensionResult<(bool, u64)> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_LEVELLING_SYSTEM") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    if experience == 0 {
        return Ok((false, 0));
    }

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    let mut level = 0u64;
    let mut xp = 0u64;
    let mut level_up = false;

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
            Logger::log_error(format!("Failed to check if schema exists. Error: {}", error.as_database_error().unwrap()),
                              "system::twilight_http_extensions::levelling_system::add_user_experience::request");

            return Err(box error);
        }
    };

    if !table_exists {
        if let Err(error) = sqlx::query(
            &format!(
                // language=SQL
                "CREATE TABLE {} (user_id TEXT, lvl TEXT, xp TEXT)",
                format!("guild_{}", guild_id.into_inner_u64())
            )
        )
            .fetch_all(&connection)
            .await {
            Logger::log_error(format!("Failed to create table. Error: {}", error.as_database_error().unwrap()),
                              "system::twilight_http_extensions::levelling_system::add_user_experience::request");

            return Err(box error);
        }
    }

    Logger::log_debug(format!("Adding experience to a user. [{} experience]", experience),
                      "system::twilight_http_extensions::levelling_system::add_user_experience::request");

    if let Ok(Some(row)) = sqlx::query(
        &format!(
            // language=SQL
            "SELECT * FROM {} WHERE user_id = $1",
            format!("guild_{}", guild_id.into_inner_u64())
        )
    )
        .bind(user_id.into_inner_u64().to_string())
        .fetch_optional(&connection)
        .await {
        level = row.get::<String, &str>("lvl").parse()?;
        xp = row.get::<String, &str>("xp").parse()?;
    }
    else {
        if let Err(error) = sqlx::query(
            &format!(
                // language=SQL
                "INSERT INTO {} (user_id, lvl, xp) VALUES ($1, $2, $3)",
                format!("guild_{}", guild_id.into_inner_u64())
            )
        )
            .bind(user_id.into_inner_u64().to_string())
            .bind(level.to_string())
            .bind(xp.to_string())
            .fetch_all(&connection)
            .await {
            Logger::log_error(format!("Failed to insert xp row. Error: {}", error.as_database_error().unwrap()),
                              "system::twilight_http_extensions::levelling_system::add_user_experience::request");

            return Err(box error);
        }
    }

    let xp_required_for_next_level = (5 * level).pow(2) + 50 * level + 100;

    xp += experience;

    if xp >= xp_required_for_next_level {
        level += 1;
        xp -= xp_required_for_next_level;

        level_up = true;
    }

    if let Err(error) = sqlx::query(
        &format!(
            // language=SQL
            "UPDATE {} SET lvl = $1, xp = $2 WHERE user_id = $3",
            format!("guild_{}", guild_id.into_inner_u64())
        )
    )
        .bind(level.to_string())
        .bind(xp.to_string())
        .bind(user_id.into_inner_u64().to_string())
        .fetch_all(&connection)
        .await {
        Logger::log_error(format!("Failed to update xp row. Error: {}", error.as_database_error().unwrap()),
                          "system::twilight_http_extensions::levelling_system::add_user_experience::request");

        return Err(box error);
    }

    Ok((level_up, level))
}
