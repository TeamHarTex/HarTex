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

crate struct RemoveUserInfraction {
    future: Option<Pending<()>>,

    guild_id: GuildId,
    user_id: UserId,
    infraction_id: String,
}

impl RemoveUserInfraction {
    crate fn new(infraction_id: String, guild_id: GuildId, user_id: UserId) -> Self {
        RemoveUserInfraction {
            future: None,

            guild_id,
            user_id,
            infraction_id,
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug(
            "Attempting to create connection to HarTexBetaGuildInfractions database.".to_string());

        self.future.replace(Box::pin(request(self.guild_id, self.user_id,
                                             self.infraction_id.clone())));

        Ok(())
    }
}

impl Future for RemoveUserInfraction {
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

unsafe impl Send for RemoveUserInfraction {}

async fn request(guild_id: GuildId, user_id: UserId, infraction_id: String) -> ClientExtensionResult<()> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_GUILDS") {
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
            "DELETE FROM inf_{}.user_{} WHERE infraction_id = $1",
            guild_id.into_inner_u64(),
            user_id.into_inner_u64()
        )
    )
        .bind::<String>(infraction_id)
        .fetch_all(&connection)
        .await;

    if let Err(error) = query_result {
        Err(box error)
    }
    else {
        Ok(())
    }
}
