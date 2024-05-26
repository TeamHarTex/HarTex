/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

//! # The Utilities Plugin
//!
//! Command List:
//! - info

use std::pin::Pin;

use async_trait::async_trait;
use hartex_database_queries::discord_frontend::queries::utilities_plugin_enabled::utilities_plugin_enabled;
use hartex_discord_commands_core::plugin;
use hartex_discord_commands_core::traits::Plugin;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_utils::DATABASE_POOL;
use miette::IntoDiagnostic;
use tokio_postgres::GenericClient;

pub mod info;

/// The utilities plugin.
#[plugin(name = "utilities")]
pub struct Utilities;

#[async_trait]
impl Plugin for Utilities {
    async fn enabled(&self, guild_id: Id<GuildMarker>) -> miette::Result<bool> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await.into_diagnostic()?;
        let client = pooled.client();

        let bool = utilities_plugin_enabled()
            .bind(client, &guild_id.to_string())
            .map(|json| json.0.get().to_string())
            .one()
            .await
            .into_diagnostic()?;

        bool.parse().into_diagnostic()
    }
}
