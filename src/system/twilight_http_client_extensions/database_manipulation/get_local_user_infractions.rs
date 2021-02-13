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
    env::*,
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
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

use crate::command_system::CommandError;
use crate::logging::logger::Logger;
use crate::system::{
    model::infractions::{
        Infraction,
        InfractionType
    },
    twilight_http_client_extensions::{
        error::ClientExtensionResult,
        Pending
    },
    twilight_id_extensions::IntoInnerU64
};

crate struct GetLocalUserInfractions {
    future: Option<Pending<Vec<Infraction>>>,

    guild_id: GuildId,
    user_id: UserId,
}

impl GetLocalUserInfractions {
    crate fn new(guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            future: None,

            guild_id,
            user_id,
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        self.future.replace(Box::pin(request(self.guild_id, self.user_id)));

        Ok(())
    }
}

impl Future for GetLocalUserInfractions {
    type Output = ClientExtensionResult<Vec<Infraction>>;

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

unsafe impl Send for GetLocalUserInfractions {}

async fn request(guild_id: GuildId, user_id: UserId) -> ClientExtensionResult<Vec<Infraction>> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_GUILD_INFRACTIONS") {
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
            "SELECT * FROM inf_{}.user_{}",
            guild_id.into_inner_u64(),
            user_id.into_inner_u64()
        )
    )
        .fetch_all(&connection)
        .await;

    match query_result {
        Ok(rows) => {
            if rows.is_empty() {
                Ok(Vec::new())
            }
            else {
                let mut infractions = Vec::<Infraction>::new();

                rows.iter().for_each(|row| {
                    let infraction_id: String = row.get("infraction_id");
                    let infraction_type = match row.get::<&str, &str>("infraction_type") {
                        "warning" => InfractionType::Warning,
                        "mute" => InfractionType::Mute,
                        "unmute" => InfractionType::Unmute,
                        "temp-mute" => InfractionType::TemporaryMute,
                        "kick" => InfractionType::Kick,
                        "ban" => InfractionType::Ban,
                        "unban" => InfractionType::Unban,
                        _ => unreachable!()
                    };
                    let reason: String = row.get("reason");

                    infractions.push(Infraction::new(infraction_id, reason, infraction_type));
                });

                Ok(infractions)
            }
        },
        Err(_) => {
            Ok(Vec::new())
        }
    }
}
