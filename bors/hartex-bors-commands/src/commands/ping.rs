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

//! # Ping command

use std::time::SystemTime;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::RepositoryClient;

/// Executes the ping command.
pub async fn ping_command<C: RepositoryClient>(
    repository: &mut GithubRepositoryState<C>,
    pr: u64,
) -> hartex_eyre::Result<()> {
    let now = SystemTime::now();
    let comment = repository.client.post_comment(pr, "🏓 Pong!").await?;

    let latencty = now.elapsed()?;

    Ok(())
}
