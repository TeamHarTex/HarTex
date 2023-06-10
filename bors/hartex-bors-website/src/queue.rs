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

//! # Bors Website - Queue Page

use std::path::PathBuf;

use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::DatabaseClient;
use hartex_log::log;
use rocket::get;
use rocket::response::content::RawHtml;
use serde::Serialize;

use crate::DATABASE;
use crate::HANDLEBARS;

#[derive(Serialize)]
struct QueueData {
    pull_requests: Vec<PullRequest>,
    repository: String,
    total: usize,
}

#[derive(Serialize)]
struct PullRequest {
    number: u64,
    assignee: String,
    approved_by: String,
    title: String,
    head_ref: String,
    url: String,
    approve_status: String,
    try_status: String,
}

/// The endpoint returning the queue page.
#[get("/queue/<repository..>")]
pub async fn queue(repository: PathBuf) -> RawHtml<String> {
    let repository_string = repository.to_string_lossy().to_string();
    let segments = repository_string.split("\\").collect::<Vec<&str>>();
    let name = GithubRepositoryName::new(segments[0], segments[1]);
    let database = DATABASE.wait().await;
    log::trace!("obtaining pull requests for repository: {name}");
    let pull_requests = database
        .get_pull_requests_for_repository(&name)
        .await
        .unwrap();

    RawHtml(
        HANDLEBARS
            .render(
                "queue",
                &QueueData {
                    pull_requests: pull_requests
                        .iter()
                        .filter(|pr| {
                            if let Some(approved_build) = &pr.approve_build && approved_build.status == BorsBuildStatus::Success {
                                return false;
                            }

                            true
                        })
                        .map(|pr| {
                            let approve_status = if let Some(approve_build) = &pr.approve_build {
                                approve_build.status.as_str().to_string()
                            } else {
                                String::new()
                            };

                            let try_status = if let Some(try_build) = &pr.try_build {
                                try_build.status.as_str().to_string()
                            } else {
                                String::new()
                            };

                            let pull_request = PullRequest {
                                number: pr.number,
                                assignee: pr.assignee.clone(),
                                approved_by: pr.approved_by.clone().unwrap_or_default(),
                                title: pr.title.clone(),
                                head_ref: pr.head_ref.clone(),
                                url: pr.url.clone(),
                                approve_status,
                                try_status,
                            };

                            pull_request
                        })
                        .collect(),
                    repository: repository_string.replace("\\", "/"),
                    total: pull_requests.len(),
                },
            )
            .unwrap(),
    )
}
