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

use sea_orm::prelude::DateTimeUtc;

type PrimaryKey = i32;

/// The status of a certain bors build.
#[derive(Debug, Eq, PartialEq)]
pub enum BorsBuildStatus {
    /// The build is pending.
    Pending,
    /// The build succeeded.
    Success,
    /// The build failed.
    Failure,
    /// The build was cancelled manually by a user.
    Cancelled,
}

/// A bors build.
pub struct BorsBuild {
    /// The identifier for this build.
    pub id: PrimaryKey,
    /// The repository.
    pub repository: String,
    /// The branch.
    pub branch: String,
    /// The hash of the commit.
    pub commit_hash: String,
    /// The build status of the build.
    pub status: BorsBuildStatus,
    /// The time when this build was created.
    pub created_at: DateTimeUtc,
}

/// A pull request that has been "indexed" by bors through approve and try commands.
pub struct BorsPullRequest {
    /// The identifier for this pull request.
    pub id: PrimaryKey,
    /// The repository.
    pub repository: String,
    /// The PR number.
    pub number: u64,
    /// The try build of this pull request, if any.
    pub try_build: Option<BorsBuild>,
    /// The time when this pull request was created.
    pub created_at: DateTimeUtc,
}
