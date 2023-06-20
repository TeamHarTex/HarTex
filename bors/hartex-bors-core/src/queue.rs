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

//! # Queue Models

use crate::models::GithubRepositoryName;

/// A pull request queue event.
#[derive(Clone, Debug)]
pub enum BorsQueueEvent {
    /// A pull request was added to the queue.
    /// 
    /// Takes in the corresponding ID of the pull request, which is a foreign key
    /// in the "enqueued" table to the "pull_request" table; and the name of the
    /// repository the pull request is associated with.
    PullRequestEnqueued(GithubRepositoryName, i32),
    /// A pull request failed its approve build, and will be de-approved.
    PullRequestFailed(GithubRepositoryName, i32),
    /// A pull request was merged and removed from the queue.
    /// 
    /// Takes in the corresponding ID of the pull request, which is a foreign key
    /// in the "enqueued" table to the "pull_request" table; and the name of the
    /// repository the pull request is associated with.
    PullRequestMerged(GithubRepositoryName, i32),
}
