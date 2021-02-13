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
    model::{
        infractions::{
            Infraction,
            InfractionType
        },
        infraction_update_type::InfractionUpdateType
    },
    twilight_http_client_extensions::{
        error::ClientExtensionResult,
        Pending
    },
    twilight_id_extensions::IntoInnerU64
};

crate struct UpdateUserInfraction {
    future: Option<Pending<Infraction>>,

    infraction_id: String,
    guild_id: GuildId,
    user_id: UserId,
    update_type: InfractionUpdateType
}

impl UpdateUserInfraction {
    crate fn new(infraction_id: String, guild_id: GuildId, user_id: UserId, update_type: InfractionUpdateType) -> Self {
        Self {
            future: None,

            infraction_id,
            guild_id,
            user_id,
            update_type
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        self.future.replace(
            Box::pin(request(
                self.infraction_id.clone(), self.update_type.clone(), self.user_id, self.guild_id)
            ));

        Ok(())
    }
}

impl Future for UpdateUserInfraction {
    type Output = ClientExtensionResult<Infraction>;

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

unsafe impl Send for UpdateUserInfraction {}

async fn request(infraction_id: String, update_type: InfractionUpdateType, user_id: UserId, guild_id: GuildId) -> ClientExtensionResult<Infraction> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_GUILD_INFRACTIONS") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    let infraction_old = match sqlx::query(
        &format!(
            // language=SQL
            "SELECT * FROM {} WHERE infraction_id = $1",
            format!("inf_{}.user_{}", guild_id.into_inner_u64(), user_id.into_inner_u64())
        )
    )
        .bind(infraction_id)
        .fetch_one(&connection)
        .await {
        Ok(row) => {
            let inf_id: String = row.get("infraction_id");
            let reason: String = row.get("reason");
            let inf_type = match row.get::<&str, &str>("infraction_type") {
                "warning" => InfractionType::Warning,
                "mute" => InfractionType::Mute,
                "unmute" => InfractionType::Unmute,
                "temp-mute" => InfractionType::TemporaryMute,
                "kick" => InfractionType::Kick,
                "ban" => InfractionType::Ban,
                "unban" => InfractionType::Unban,
                _ => unreachable!()
            };

            Infraction::new(inf_id, reason, inf_type)
        },
        Err(error) => {
            return Err(box error)
        }
    };

    match update_type {
        InfractionUpdateType::Reason { new_reason } => {
            sqlx::query(
                &format!(
                    // language=SQL
                    "UPDATE {} SET reason = $1 WHERE infraction_id = $2",
                    format!("inf_{}.user_{}", guild_id.into_inner_u64(), user_id.into_inner_u64())
                )
            )
                .bind(new_reason)
                .bind(infraction_old.clone().infraction_id)
                .fetch_one(&connection)
                .await?;
        }
    };

    Ok(infraction_old)
}
