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

//! # Bors GitHub API Interaction

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(async_fn_in_trait)]

use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::BorsState;
use hartex_bors_core::RepositoryClient;
use hartex_bors_database::client::SeaORMDatabaseClient;
use hartex_eyre::eyre::Report;
use hartex_log::log;
use jsonwebtoken::EncodingKey;
use octocrab::models::issues::Comment;
use octocrab::models::pulls::PullRequest;
use octocrab::models::AppId;
use octocrab::models::Repository;
use octocrab::models::{App, CommentId};
use octocrab::Octocrab;
use secrecy::ExposeSecret;
use secrecy::SecretVec;
use std::collections::HashMap;

pub mod operations;
mod repositories;
pub mod webhook;

/// State of the bors Github application
#[allow(dead_code)]
pub struct GithubBorsState {
    application: App,
    client: Octocrab,
    database: SeaORMDatabaseClient,
    repositories: RepositoryMap,
}

impl GithubBorsState {
    /// Load the Github application state for bors.
    pub async fn load(
        application_id: AppId,
        database: SeaORMDatabaseClient,
        private_key: SecretVec<u8>,
    ) -> hartex_eyre::Result<Self> {
        log::trace!("obtaining private key");
        let key = EncodingKey::from_rsa_pem(private_key.expose_secret().as_ref())?;

        log::trace!("building github client");
        let client = Octocrab::builder().app(application_id, key).build()?;

        log::trace!("obtaining github application");
        let application = client.current().app().await?;

        let repositories = repositories::load_repositories(&client).await?;
        Ok(Self {
            application,
            client,
            database,
            repositories,
        })
    }

    pub fn database_mut(&mut self) -> &mut SeaORMDatabaseClient {
        &mut self.database
    }
}

impl BorsState<GithubRepositoryClient> for GithubBorsState {
    fn comment_posted_by_bors(&self, comment: Comment) -> bool {
        comment.user.login == "bors-for-hartex[bot]"
    }

    fn get_repository_state_mut(
        &mut self,
        repository: &GithubRepositoryName,
    ) -> Option<&mut GithubRepositoryState<GithubRepositoryClient>> {
        self.repositories.get_mut(repository)
    }
}

/// A Github repository client.
pub struct GithubRepositoryClient {
    /// Octocrab client.
    client: Octocrab,
    /// The name of the repository.
    repository_name: GithubRepositoryName,
    /// The repository.
    repository: Repository,
}

impl GithubRepositoryClient {
    /// Returns a reference to the Octocrab client.
    pub fn client(&self) -> &Octocrab {
        &self.client
    }

    /// Returns a reference to the repository.
    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}

impl RepositoryClient for GithubRepositoryClient {
    fn repository_name(&self) -> &GithubRepositoryName {
        &self.repository_name
    }

    async fn edit_comment(
        &mut self,
        comment_id: CommentId,
        text: &str,
    ) -> hartex_eyre::Result<Comment> {
        self.client
            .issues(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .update_comment(comment_id, text)
            .await
            .map_err(Report::new)
    }

    async fn get_pull_request(&mut self, pr: u64) -> hartex_eyre::Result<PullRequest> {
        self.client
            .pulls(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .get(pr)
            .await
            .map_err(Report::new)
    }

    async fn post_comment(&mut self, pr: u64, text: &str) -> hartex_eyre::Result<Comment> {
        self.client
            .issues(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .create_comment(pr, text)
            .await
            .map_err(Report::new)
    }

    async fn set_branch_to_revision(
        &mut self,
        branch: &str,
        revision: &str,
    ) -> hartex_eyre::Result<()> {
        operations::set_branch_to_revision(self, branch.to_string(), revision.to_string()).await
    }
}

type RepositoryMap = HashMap<GithubRepositoryName, RepositoryState>;
type RepositoryState = GithubRepositoryState<GithubRepositoryClient>;
