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

//! # The General Plugin
//!
//! Command list:
//! - about
//! - contributors

use async_trait::async_trait;
use hartex_discord_commands_core::plugin;
use hartex_discord_commands_core::traits::Plugin;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::Id;
use miette::IntoDiagnostic;
use tokio_postgres::GenericClient;

pub mod about;
pub mod contributors;

/// The general plugin.
#[plugin(name = "general")]
pub struct General;

#[async_trait]
impl Plugin for General {
    async fn enabled(&self, _: Id<GuildMarker>) -> miette::Result<bool> {
        Ok(true)
    }
}
