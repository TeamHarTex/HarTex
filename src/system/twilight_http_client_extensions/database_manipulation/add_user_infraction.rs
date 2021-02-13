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
    model::infractions::InfractionType,
    twilight_http_client_extensions::{
        error::ClientExtensionResult,
        Pending
    },
    twilight_id_extensions::IntoInnerU64
};

crate struct AddUserInfraction {
    future: Option<Pending<()>>,

    infraction_id: String,
    guild_id: GuildId,
    user_id: UserId,
    reason: String,
    infraction_type: InfractionType,
}

impl AddUserInfraction {
    crate fn new(infraction_id: String, guild_id: GuildId, user_id: UserId, reason: String,
               infraction_type: InfractionType) -> Self {
        AddUserInfraction {
            future: None,

            infraction_id,
            guild_id,
            user_id,
            reason,
            infraction_type,
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug("Attempting to create connection to HarTexBetaGuildInfractions database.".to_string());

        self.future.replace(Box::pin(request(self.infraction_id.clone(), self.guild_id, self.user_id,
                                             self.reason.clone(), self.infraction_type)));

        Ok(())
    }
}

impl Future for AddUserInfraction {
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

unsafe impl Send for AddUserInfraction {}

async fn request(infraction_id: String, guild_id: GuildId, user_id: UserId, reason: String,
                 infraction_type: InfractionType) -> ClientExtensionResult<()> {
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
        if let Err(error) = sqlx::query(
            // language=SQL
            "CREATE SCHEMA $1; --"
        )
            .bind(format!("inf_{}", guild_id.0))
            .fetch_one(&connection)
            .await {
            Logger::log_error(format!("Could not create schema. Error: {}", error.as_database_error().unwrap()));

            return Err(box error);
        }
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
        if let Err(error) = sqlx::query(

            &format!(
                // language=SQL
                "CREATE TABLE {} (infraction_id TEXT, reason TEXT, infraction_type TEXT)",
                format!("inf_{}.user_{}", guild_id.0, user_id.0)
            )
        )
            .fetch_all(&connection)
            .await {
            Logger::log_error(format!("Could not create table. Error: {}", error.as_database_error().unwrap()));

            return Err(box error)
        }
    }

    if let Err(error) = sqlx::query(
        &format!(
            // language=SQL
            "INSERT INTO {} (infraction_id, reason, infraction_type) VALUES ($1, $2, $3)",
            format!("inf_{}.user_{}", guild_id.0, user_id.0))
    )
        .bind(infraction_id.clone())
        .bind(reason.clone())
        .bind(infraction_type.to_string())
        .fetch_all(&connection)
        .await {
        Logger::log_error(format!("Could not delete infraction. Error: {}", error));

        return Err(box error)
    }

    Ok(())
}
