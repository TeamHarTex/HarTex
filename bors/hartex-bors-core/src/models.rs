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

//! # Utility Models

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use hartex_eyre::eyre::Report;
use octocrab::models::Repository;
use octocrab::models::RunId;
use sea_orm::prelude::DateTimeUtc;

use crate::PermissionResolver;
use crate::RepositoryClient;

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

/// A bors workflow.
pub struct BorsWorkflow {
    /// The identifier for this workflow.
    pub id: PrimaryKey,
    /// The build this workflow is associated to.
    pub build: BorsBuild,
    /// The name of the workflow.
    pub name: String,
    /// The url of the workflow.
    pub url: String,
    /// The run id of the workflow.
    pub run_id: RunId,
    /// The type of the workflow.
    pub workflow_type: BorsWorkflowType,
    /// The status of the workflow.
    pub workflow_status: BorsWorkflowStatus,
    /// The time when this workflow was created.
    pub created_at: DateTimeUtc,
}

/// The bors workflow status.
#[derive(Debug, Eq, PartialEq)]
pub enum BorsWorkflowStatus {
    /// The workflow is running.
    Pending,
    /// The workflow has succeeded.
    Success,
    /// The workflow has failed.
    Failure,
}

/// The bors workflow type.
#[derive(Debug, Eq, PartialEq)]
pub enum BorsWorkflowType {
    /// A GitHub workflow.
    GitHub,
    /// An external workflow.
    External,
}

/// Name of a Github repository
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GithubRepositoryName {
    owner: String,
    repository: String,
}

impl GithubRepositoryName {
    /// Constrct a new repository name.
    pub fn new(owner: &str, repository: &str) -> Self {
        Self {
            owner: owner.to_lowercase(),
            repository: repository.to_lowercase(),
        }
    }

    pub fn new_from_repository(repository: Repository) -> hartex_eyre::Result<Self> {
        let name = &repository.name;
        let Some(owner) = repository.owner
            .as_ref()
            .map(|author| &author.login) else {
            return Err(Report::msg(format!("repository {name} seemingly has no owner")));
        };

        Ok(Self::new(owner, name))
    }

    /// Obtain repository owner
    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    /// Obtain repository name
    pub fn repository(&self) -> &str {
        self.repository.as_str()
    }
}

impl Display for GithubRepositoryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}/{}", self.owner, self.repository)
    }
}

/// A repository state.
pub struct GithubRepositoryState<C: RepositoryClient> {
    /// The repository name.
    pub repository: GithubRepositoryName,
    /// The client for this repository.
    pub client: C,
    /// The permission resolver for this repository.
    pub permission_resolver: Box<dyn PermissionResolver>,
}

/// The type of permission.
#[non_exhaustive]
pub enum Permission {
    /// Permission to try builds.
    ///
    /// bors try
    TryBuild,
}

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::TryBuild => write!(f, "trybuild"),
        }
    }
}
