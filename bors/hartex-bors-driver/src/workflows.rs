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

use hartex_bors_commands::commands::try::TRY_BRANCH_NAME;
use hartex_bors_core::DatabaseClient;
use octocrab::models::workflows::Run;
use hartex_log::log;

pub(crate) fn workflow_started(
    database: &mut dyn DatabaseClient,
    run: Run
) -> hartex_eyre::Result<()> {
    if !is_relevant_branch(&run.head_branch) {
        return Ok(());
    }

    log::trace!(
        "handling workflow started (name={}, url={}, branch={}, commit={})",
        run.name,
        run.url,
        run.head_branch,
        run.head_sha,
    );

    todo!()
}

fn is_relevant_branch(branch: &str) -> bool {
    [TRY_BRANCH_NAME].contains(&branch)
}
