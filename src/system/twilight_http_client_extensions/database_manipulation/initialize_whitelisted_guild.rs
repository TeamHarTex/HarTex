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
    io::Cursor,
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

use twilight_http::{
    Client
};

use twilight_model::{
    id::GuildId
};

use quick_xml::Writer;

use crate::command_system::{
    CommandError
};

use crate::logging::logger::Logger;

use crate::xml_deserialization::{
    plugin_management::{
        command::{
            infractions::{
                MuteCommand
            }
        },
        InfractionsPlugin,
        Plugins,
    },
    BotConfig,
    BotCustomization,
    Dashboard,
    RolePermissionLevels,
    User
};

use super::{
    super::{
        error::ClientExtensionResult,
        Pending
    },
};

use crate::system::twilight_id_extensions::IntoInnerU64;

crate struct InitializeWhitelistedGuild {
    future: Option<Pending<BotConfig>>,

    guild_id: GuildId,

    http: Client
}

impl InitializeWhitelistedGuild {
    crate fn new(http: Client, guild_id: GuildId) -> Self {
        Self {
            future: None,

            guild_id,

            http
        }
    }

    fn start(&mut self) -> ClientExtensionResult<()> {
        Logger::log_debug(
            "Attempting to create connection to HarTexBetaGuildConfiguration database.".to_string());

        self.future.replace(Box::pin(request(self.guild_id.clone(), self.http.clone())));

        Ok(())
    }
}

impl Future for InitializeWhitelistedGuild {
    type Output = ClientExtensionResult<BotConfig>;

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

unsafe impl Send for InitializeWhitelistedGuild {}

async fn request(guild_id: GuildId, http: Client) -> ClientExtensionResult<BotConfig> {
    let database_credentials = if let Ok(credentials) = var("PGSQL_CREDENTIALS_GUILD_CONFIGURATION") {
        credentials
    }
    else {
        return Err(box CommandError("Credentials is none.".to_string()))
    };

    let connection = PgPool::connect(
        &database_credentials
    ).await?;

    let guild_request = http.guild(guild_id).await?;

    if let Some(guild) = guild_request {
        let mut writer =
            Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
        let config = BotConfig {
            dashboard: Dashboard {
                users: vec![User {
                    id: guild.owner_id.into_inner_u64(),
                    permission_level: 3
                }]
            },
            bot_customization: BotCustomization::default(),
            role_permission_levels: RolePermissionLevels::default(),
            plugins: Plugins {
                infractions_plugin: InfractionsPlugin {
                    mute_command: MuteCommand {
                        muted_role: None,
                        role_to_remove: None
                    }
                }
            }
        };

        quick_xml::se::to_writer(writer.inner().get_mut(), &config)?;

        let bytes = writer.into_inner().into_inner();

        // Assuming when this is executed, all pre-execution checks are done: the schema and tables for the guild does NOT exist.
        
        
        let result = sqlx::query(
            &format!(
                // language=SQL
                "CREATE SCHEMA {}",
                format!("guild_{}", guild_id.into_inner_u64())
            )
        )
            .fetch_all(&connection)
            .await?;

        let result = sqlx::query(
            &format!(
                // language=SQL
                "CREATE TABLE {}.plain_config (config TEXT)",
                format!("guild_{}", guild_id.into_inner_u64())
            )
        )
            .fetch_all(&connection)
            .await;

        if let Err(error) = result {
            return Err(
                box CommandError(
                    format!(
                        "Could not create table due to an error: '{}'", error.into_database_error().unwrap()
                    )
                )
            );
        }

        if let Err(error) = sqlx::query(
            &format!(
                // language=SQL
                "INSERT INTO {} (config) VALUES ($1)",
                format!("guild_{}.plain_config", guild_id.into_inner_u64())
            )
        )
            .bind(base64::encode(bytes))
            .fetch_all(&connection)
            .await {
            return Err(
                box CommandError(
                    format!(
                        "Could not setup guild due to an error: '{}'", error.into_database_error().unwrap()
                    )
                )
            );
        }

        Ok(config)
    }
    else {
        Err(box CommandError("Guild ID is invalid.".to_string()))
    }
}
