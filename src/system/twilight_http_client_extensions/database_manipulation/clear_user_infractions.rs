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

crate struct ClearUserInfractions {
    future: Option<Pending<()>>,

    guild_id: GuildId,
    user_id: UserId
}

impl ClearUserInfractions {
    crate fn new(guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            future: None,

            guild_id,
            user_id
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug("Attempting to create connection to HarTexBetaGuildInfractions database.".to_string());

        self.future.replace(Box::pin(request(self.guild_id, self.user_id)));

        Ok(())
    }
}

impl Future for ClearUserInfractions {
    type Output = ClientExtensionResult<()>;

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

unsafe impl Send for ClearUserInfractions {}

async fn request(guild_id: GuildId, user_id: UserId) -> ClientExtensionResult<()> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_GUILD_INFRACTIONS") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    Logger::log_debug(
        format!(
            "Making query to database. [Checking if guild schema exists in the database: {}]", guild_id));

    let schema_query_result: SqlxResult<PgRow> = sqlx::query(
        // language=SQL
        "SELECT EXISTS (SELECT * FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = $1); --"
    )
        .bind::<String>(format!("inf_{}", guild_id.into_inner_u64()))
        .fetch_one(&connection).await;

    let schema_exists = match schema_query_result {
        Ok(row) => {
            Logger::log_verbose("Successfully made query.");

            let exists: bool = row.get("exists");

            exists
        },
        Err(error) => {
            Logger::log_error(
                format!("Failed check if schema exists. Error: {}", error.as_database_error().unwrap()));

            return Err(box error);
        }
    };

    if !schema_exists {
        return Ok(());
    }

    let table_query_result: SqlxResult<PgRow> = sqlx::query(
        // language=SQL
        "SELECT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = CONCAT('user_', $1) AND TABLE_SCHEMA = CONCAT('inf_', $2))"
    )
        .bind::<String>(user_id.0.to_string())
        .bind(guild_id.0.to_string())
        .fetch_one(&connection)
        .await;
    let table_exists = match table_query_result {
        Ok(row) => {
            Logger::log_verbose("Successfully made query.");

            let exists: bool = row.get("exists");

            exists
        },
        Err(error) => {
            Logger::log_error(format!("Failed to check if table exists. Error: {}", error.as_database_error().unwrap()));

            return Err(box error);
        }
    };

    if !table_exists {
        return Ok(());
    }

    if let Err(error) = sqlx::query(
        &format!(
            // language=SQL
            "DELETE FROM {}",
            format!("inf_{}.user_{}", guild_id.0, user_id.0))
    )
        .fetch_all(&connection)
        .await {
        return Err(box error)
    }

    todo!()
}
