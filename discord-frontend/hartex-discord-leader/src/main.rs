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

use std::sync::LazyLock;

use git_version::git_version;
use hartex_discord_utils::{TOKEN, error::HarTexResult};
use mimalloc::MiMalloc;
use tracing::subscriber;

mod shards;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

#[tokio::main]
pub async fn main() -> HarTexResult<()> {
    subscriber::set_global_default(hartex_tracing::subscriber())?;

    tracing::info!(
        "HarTex {} ({} {})",
        env!("CARGO_PKG_VERSION"),
        git_version!(),
        env!("CARGO_BUILD_DATE")
    );
    tracing::debug!("starting up...");

    if let Err(report) = LazyLock::force(&TOKEN) {
        tracing::error!("`TOKEN` environment variable error: {report}");
        return Err(report.clone());
    }

    Ok(())
}
