/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;

use hartex_discord_core::discord::model::application::interaction::InteractionType;
use hartex_discord_core::discord::model::gateway::event::DispatchEvent;
use hartex_discord_core::discord::model::gateway::event::GatewayEvent;
use hartex_log::log;
use miette::IntoDiagnostic;
use sqlx::postgres::PgConnection;
use sqlx::postgres::PgTypeInfo;
use sqlx::prelude::Connection;
use sqlx::prelude::Executor;
use sqlx::prelude::Statement;
use sqlx::types::chrono::Utc;

/// Invoke a corresponding event callback for an event,
pub async fn invoke(event: GatewayEvent, shard: u8) -> miette::Result<()> {
    #[allow(clippy::collapsible_match)]
    match event {
        GatewayEvent::Dispatch(seq, dispatch) => match dispatch {
            DispatchEvent::InteractionCreate(interaction_create)
                if interaction_create.kind == InteractionType::ApplicationCommand =>
            {
                crate::interaction::application_command(interaction_create).await
            }
            DispatchEvent::Ready(ready) => {
                log::info!(
                        "{}#{} (shard {shard}) has received READY payload from Discord (gateway v{}) (sequence {seq})",
                        ready.user.name,
                        ready.user.discriminator,
                        ready.version
                    );

                let mut connection =
                    PgConnection::connect(&env::var("API_PGSQL_URL").unwrap()).await.into_diagnostic()?;
                let statement = connection
                        .prepare_with(
                            include_str!("../../../database-queries/start-timestamp/insert-into-on-conflict-update.sql"),
                            &[PgTypeInfo::with_name("TEXT"), PgTypeInfo::with_name("TIMESTAMPTZ")],
                        )
                        .await
                    .into_diagnostic()?;

                statement
                    .query()
                    .bind("HarTex Nightly")
                    .bind(Utc::now())
                    .execute(&mut connection)
                    .await
                    .into_diagnostic()?;

                Ok(())
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}
