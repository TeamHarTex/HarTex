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

//! # The Pull Request Queue Processor

use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::queue::BorsQueueEvent;
use hartex_bors_core::DatabaseClient;
use hartex_log::log;
use tokio::sync::mpsc::Receiver;

/// Background task processing the queue.
#[allow(dead_code)]
pub async fn queue_processor(
    mut rx: Receiver<BorsQueueEvent>,
    database: Box<dyn DatabaseClient>,
) -> hartex_eyre::Result<()> {
    while let Some(event) = rx.recv().await {
        match event {
            BorsQueueEvent::PullRequestEnqueued(name, id) => {
                log::trace!("pull request with id {id} in pull_request table has been enqueued");

                let queue = database
                    .get_enqueued_pull_requests_for_repository(&name)
                    .await?;
                if queue.iter().any(|pull_request| {
                    let Some(ref approve_build) = pull_request.pull_request.approve_build else {
                        unreachable!();
                    };

                    approve_build.status == BorsBuildStatus::Pending
                }) {
                    // continue waiting
                    continue;
                }
            }
        }
    }

    Ok(())
}
