/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use std::sync::LazyLock;

use color_eyre::eyre::Result;
use hartex_version::version;

use crate::{app::App, keybinds::KEYBINDS};

mod app;
mod component;
mod errorhandler;
mod keybinds;
mod logging;
mod tui;

#[tokio::main]
pub async fn main() -> Result<()> {
    errorhandler::initialize()?;
    logging::initialize()?;

    tracing::info!("HarTex Management TUI: {}", version());

    tracing::trace!("loading keybinds");
    LazyLock::force(&KEYBINDS);

    tracing::trace!("creating and running app");
    let mut app = App::new(60.0, 4.0);
    app.run().await?;

    tracing::warn!("stopping HarTex Management TUI");

    Ok(())
}
