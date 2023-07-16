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

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str;
use std::str::FromStr;

use http::StatusCode;
use http::Uri;
use http_body::Body;
use miette::Diagnostic;
use miette::IntoDiagnostic;
use miette::Report;
use octocrab::params::repos::Reference;
use serde_json::Value;

use crate::GithubRepositoryClient;

/// Errors when updating a branch.
#[allow(dead_code)]
#[derive(Debug, Diagnostic)]
pub enum UpdateBranchError {
    /// The branch to update does not exist.
    BranchNotFound(String),
    Unknown,
}

impl UpdateBranchError {
    pub fn is_branch_not_found(&self) -> bool {
        let Self::BranchNotFound(_) = self else {
            return false;
        };

        true
    }
}

impl Display for UpdateBranchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BranchNotFound(branch) => write!(f, "branch {branch} is not found"),
            _ => write!(f, "unknown error"),
        }
    }
}

impl Error for UpdateBranchError {}

/// Set a branch to a specific commit revision.
pub async fn set_branch_to_revision(
    repository: &GithubRepositoryClient,
    name: String,
    commit_hash: String,
) -> miette::Result<()> {
    match update_branch(repository, name.clone(), commit_hash.clone()).await {
        Ok(_) => Ok(()),
        Err(error) if let Some(error) = error.downcast_ref::<UpdateBranchError>()
            && error.is_branch_not_found() => {
            match create_branch(repository, name.clone(), commit_hash).await {
                Ok(_) => Ok(()),
                error => error,
            }
        },
        error => error,
    }
}

async fn create_branch(
    repository: &GithubRepositoryClient,
    name: String,
    commit_hash: String,
) -> miette::Result<()> {
    repository
        .client
        .repos(
            repository.repository_name.owner(),
            repository.repository_name.repository(),
        )
        .create_ref(&Reference::Branch(name), commit_hash)
        .await
        .into_diagnostic()?;

    Ok(())
}

pub async fn merge_branches(
    repository: &GithubRepositoryClient,
    base: &str,
    head: &str,
    commit_message: &str,
) -> miette::Result<String> {
    let url = repository
        .repository
        .merges_url
        .as_ref()
        .map(|url| url.to_string())
        .unwrap_or_else(|| {
            format!(
                "https://api.github.com/repos/{}/{}/merges",
                repository.repository_name.owner(),
                repository.repository_name.repository()
            )
        });

    let mut response = repository
        .client
        ._post(
            &url,
            Some(&serde_json::json!({
                "base": base,
                "head": head,
                "commit_message": commit_message
            })),
        )
        .await
        .into_diagnostic()?;

    let status = response.status();
    let mut full = String::new();
    while let Some(result) = response.body_mut().data().await {
        full.push_str(str::from_utf8(&result.into_diagnostic()?).into_diagnostic()?);
    }

    match status {
        StatusCode::CREATED => {
            let value = serde_json::from_str::<Value>(&full).into_diagnostic()?;
            let Value::String(sha) = value["sha"].clone() else {
                unreachable!()
            };

            Ok(sha)
        }
        StatusCode::NOT_FOUND => Err(Report::msg("branch not found")),
        StatusCode::CONFLICT => Err(Report::msg("merge conflict occurred")),
        StatusCode::NO_CONTENT => Err(Report::msg("branches have already been merged")),
        _ => Err(Report::msg("unknown error")),
    }
}

async fn update_branch(
    repository: &GithubRepositoryClient,
    name: String,
    commit_hash: String,
) -> miette::Result<()> {
    let uri = Uri::from_str(&format!(
        "https://api.github.com/repos/{}/{}/git/refs/{}",
        repository.repository_name.owner(),
        repository.repository_name.repository(),
        Reference::Branch(name.clone()).ref_url()
    ))
    .into_diagnostic()?;

    let result = repository
        .client
        ._patch(
            uri,
            Some(&serde_json::json!({
                "sha": commit_hash.clone(),
                "force": true
            })),
        )
        .await
        .into_diagnostic()?;

    match result.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Report::new(UpdateBranchError::BranchNotFound(name))),
    }
}
