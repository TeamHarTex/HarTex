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

//! Utility Operations

use std::str::FromStr;

use hartex_eyre::eyre::Report;
use http::StatusCode;
use http::Uri;
use octocrab::params::repos::Reference;

use crate::GithubRepositoryClient;

/// Errors when updating a branch.
#[derive(Debug)]
pub enum UpdateBranchError {
    /// The branch to update does not exist.
    BranchNotFound(String),
}

/// Set a branch to a specific commit revision.
pub async fn set_branch_to_revision(
    repository: &GithubRepositoryClient,
    name: String,
    commit_hash: String,
) -> hartex_eyre::Result<()> {
    match update_branch(repository, name.clone(), commit_hash.clone()).await {
        Ok(_) => Ok(()),
        Err(UpdateBranchError::BranchNotFound(_)) => {
            match create_branch(repository, name.clone(), commit_hash).await {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

async fn create_branch(
    repository: &GithubRepositoryClient,
    name: String,
    commit_hash: String,
) -> hartex_eyre::Result<()> {
    repository
        .client
        .repos(
            repository.repository_name.owner(),
            repository.repository_name.repository(),
        )
        .create_ref(&Reference::Branch(name), commit_hash)
        .await?;

    Ok(())
}

async fn update_branch(
    repository: &GithubRepositoryClient,
    name: String,
    commit_hash: String,
) -> hartex_eyre::Result<()> {
    let uri = Uri::from_str(&format!(
        "https://api.github.com/repos/{}/{}/git/refs/{}",
        repository.repository_name.owner(),
        repository.repository_name.repository(),
        Reference::Branch(name.clone()).ref_url()
    ))?;

    let result = repository
        .client
        ._patch(
            uri,
            Some(&serde_json::json!({
                "sha": commit_hash.as_ref(),
                "force": true
            })),
        )
        .await?;

    match result.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Report::new(UpdateBranchError::BranchNotFound(name))),
    }
}
