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

use dashmap::DashMap;

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

use twilight_http::Client;

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
        GetLocalUserInfractions,
        Pending
    },
    twilight_id_extensions::IntoInnerU64
};

crate struct GetGuildInfractions {
    future: Option<Pending<DashMap<UserId, Vec<Infraction>>>>,

    guild_id: GuildId,
    http_client: Client
}

impl GetGuildInfractions {
    crate fn new(guild_id: GuildId, http_client: Client) -> Self {
        Self {
            future: None,

            guild_id,
            http_client
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        self.future.replace(Box::pin(request(self.guild_id, self.http_client.clone())));

        Ok(())
    }
}

impl Future for GetGuildInfractions {
    type Output = ClientExtensionResult<DashMap<UserId, Vec<Infraction>>>;

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

unsafe impl Send for GetGuildInfractions {}

async fn request(guild_id: GuildId, http_client: Client) -> ClientExtensionResult<DashMap<UserId, Vec<Infraction>>> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_GUILD_INFRACTIONS") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    let tables_query_result = sqlx::query(
        &format!(
            // language=SQL
            "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = $1"
        )
    )
        .bind::<String>(format!("inf_{}", guild_id.into_inner_u64()))
        .fetch_all(&connection)
        .await;

    match tables_query_result {
        Ok(rows) => {
            let infraction_map = DashMap::<UserId, Vec<Infraction>>::new();

            for row in rows {
                let name: String = row.get("table_name");
                let user_id = UserId::from(name.get(5..).unwrap().parse::<u64>().unwrap());
                let infractions = http_client.clone().get_local_user_infractions(guild_id, user_id).await?;

                infraction_map.insert(user_id, infractions);
            }

            Ok(infraction_map)
        },
        Err(error) => {
            Err(box error)
        }
    }
}
