/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # Configuration Provider
//!
//! This crate contains APIs for easier retrieval of specific parts of a configuration for servers.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_database_queries::queries::configuration::plugin_enabled::PluginEnabled;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use miette::IntoDiagnostic;

/// The configuration provide for fetching configuration.
pub struct ConfigurationProvider;

impl ConfigurationProvider {
    /// Queries whether a specific plugin is enabled for a certain guild.
    #[allow(clippy::missing_errors_doc)]
    pub async fn plugin_enabled(
        guild_id: Id<GuildMarker>,
        plugin: impl Into<String>,
    ) -> miette::Result<bool> {
        PluginEnabled::bind(plugin.into(), guild_id.to_string())
            .executor()
            .await
            .into_diagnostic()?
            .one()
            .await
            .into_diagnostic()
    }
}
