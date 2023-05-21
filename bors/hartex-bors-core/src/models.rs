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
use octocrab::models::Repository;
use hartex_eyre::eyre::Report;

use crate::PermissionResolver;
use crate::RepositoryClient;

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
