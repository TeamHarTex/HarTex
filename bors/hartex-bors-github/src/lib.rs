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
#![feature(if_let_guard)]
#![feature(let_chains)]

use std::collections::HashMap;
use std::str;

use futures::future;
use hartex_bors_core::models::Check;
use hartex_bors_core::models::CheckStatus;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::queue::BorsQueueEvent;
use hartex_bors_core::BorsState;
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use hartex_bors_database::client::SeaORMDatabaseClient;
use hartex_log::log;
use http::StatusCode;
use http_body::Body;
use jsonwebtoken::EncodingKey;
use miette::IntoDiagnostic;
use miette::Report;
use octocrab::models::issues::Comment;
use octocrab::models::pulls::PullRequest;
use octocrab::models::repos::Object;
use octocrab::models::App;
use octocrab::models::AppId;
use octocrab::models::CommentId;
use octocrab::models::Label;
use octocrab::models::Repository;
use octocrab::models::RunId;
use octocrab::params::repos::Reference;
use octocrab::Octocrab;
use secrecy::ExposeSecret;
use secrecy::SecretVec;
use serde::Deserialize;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub mod messages;
mod operations;
mod repositories;
pub mod webhook;

/// State of the bors Github application
#[allow(dead_code)]
pub struct GithubBorsState {
    application: App,
    client: Octocrab,
    pub database: SeaORMDatabaseClient,
    repositories: RepositoryMap,
    sender: Sender<BorsQueueEvent>,
}

impl GithubBorsState {
    /// Load the Github application state for bors.
    pub async fn load(
        application_id: AppId,
        database: SeaORMDatabaseClient,
        private_key: SecretVec<u8>,
    ) -> miette::Result<(Self, Receiver<BorsQueueEvent>)> {
        log::trace!("obtaining private key");
        let key =
            EncodingKey::from_rsa_pem(private_key.expose_secret().as_ref()).into_diagnostic()?;

        log::trace!("building github client");
        let client = Octocrab::builder()
            .app(application_id, key)
            .build()
            .into_diagnostic()?;

        log::trace!("obtaining github application");
        let application = client.current().app().await.into_diagnostic()?;

        let (tx, rx) = channel(15);
        let repositories = repositories::load_repositories(&client, &database).await?;
        Ok((
            Self {
                application,
                client,
                database,
                repositories,
                sender: tx,
            },
            rx,
        ))
    }
}

impl BorsState<GithubRepositoryClient> for GithubBorsState {
    fn comment_posted_by_bors(&self, comment: Comment) -> bool {
        comment.user.login == "bors-for-hartex[bot]"
    }

    fn get_repository_state(
        &self,
        repository: &GithubRepositoryName,
    ) -> Option<(
        &GithubRepositoryState<GithubRepositoryClient>,
        &dyn DatabaseClient,
        Sender<BorsQueueEvent>,
    )> {
        self.repositories.get(repository).map(|repo| {
            (
                repo,
                (&self.database) as &dyn DatabaseClient,
                self.sender.clone(),
            )
        })
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

    async fn cancel_workflows(&self, run_ids: Vec<RunId>) -> miette::Result<()> {
        let actions = self.client.actions();

        future::join_all(run_ids.into_iter().map(|id| {
            actions.cancel_workflow_run(
                self.repository_name.owner(),
                self.repository_name.repository(),
                id,
            )
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, octocrab::Error>>()
        .into_diagnostic()?;

        Ok(())
    }

    async fn delete_branch(&self, branch: &str) -> miette::Result<()> {
        let mut response = self
            .client
            ._delete::<()>(
                format!(
                    "https://api.github.com/repos/{}/{}/git/refs/{}",
                    self.repository_name.owner(),
                    self.repository_name.repository(),
                    Reference::Branch(branch.to_string()).ref_url()
                ),
                None,
            )
            .await
            .into_diagnostic()?;

        if response.status() != StatusCode::NO_CONTENT {
            let mut full = String::new();
            while let Some(result) = response.body_mut().data().await {
                full.push_str(str::from_utf8(&result.into_diagnostic()?).into_diagnostic()?);
            }

            return Err(Report::msg(format!("failed to delete branch: {full}")));
        }

        Ok(())
    }

    async fn edit_comment(&self, comment_id: CommentId, text: &str) -> miette::Result<Comment> {
        self.client
            .issues(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .update_comment(comment_id, text)
            .await
            .into_diagnostic()
    }

    async fn get_checks_for_commit(
        &self,
        branch: &str,
        commit_hash: &str,
    ) -> miette::Result<Vec<Check>> {
        let mut response = self
            .client
            ._get(format!(
                "https://api.github.com/repos/{}/{}/commits/{}/check-suites",
                self.repository_name.owner(),
                self.repository_name.repository(),
                commit_hash
            ))
            .await
            .into_diagnostic()?;

        #[derive(Deserialize)]
        struct CheckSuitePayload<'a> {
            conclusion: Option<&'a str>,
            head_branch: &'a str,
        }

        #[derive(Deserialize)]
        struct CheckSuiteResponse<'a> {
            #[serde(borrow)]
            check_suites: Vec<CheckSuitePayload<'a>>,
        }

        let mut full = String::new();
        while let Some(result) = response.body_mut().data().await {
            full.push_str(str::from_utf8(&result.into_diagnostic()?).into_diagnostic()?);
        }

        let response = serde_json::from_str::<CheckSuiteResponse<'_>>(&full).into_diagnostic()?;
        let suites = response
            .check_suites
            .into_iter()
            .filter(|suite| suite.head_branch == branch)
            .map(|suite| Check {
                status: match suite.conclusion {
                    Some(status) => match status {
                        "success" => CheckStatus::Success,
                        "failure" | "neutral" | "cancelled" | "skipped" | "timed_out"
                        | "action_required" | "startup_failure" | "stale" => CheckStatus::Failure,
                        _ => {
                            log::warn!(
                                "Received unknown check suite status for {}/{}: {status}",
                                self.repository_name,
                                commit_hash,
                            );

                            CheckStatus::Pending
                        }
                    },
                    None => CheckStatus::Pending,
                },
            })
            .collect();

        Ok(suites)
    }

    async fn get_label(&self, name: &str) -> miette::Result<Label> {
        self.client
            .issues(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .get_label(name)
            .await
            .into_diagnostic()
    }

    async fn set_labels_of_pull_request(&self, labels: Vec<String>, pr: u64) -> miette::Result<()> {
        let mut response = self
            .client
            ._put(
                format!(
                    "https://api.github.com/repos/{}/{}/issues/{pr}/labels",
                    self.repository_name.owner(),
                    self.repository_name.repository(),
                ),
                Some(&serde_json::json!({
                    "labels": labels,
                })),
            )
            .await
            .into_diagnostic()?;

        if response.status() != StatusCode::OK {
            let mut full = String::new();
            while let Some(result) = response.body_mut().data().await {
                full.push_str(str::from_utf8(&result.into_diagnostic()?).into_diagnostic()?);
            }

            return Err(Report::msg(full));
        }

        Ok(())
    }

    async fn get_pull_request(&self, pr: u64) -> miette::Result<PullRequest> {
        self.client
            .pulls(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .get(pr)
            .await
            .into_diagnostic()
    }

    async fn merge_branches(
        &self,
        base: &str,
        head: &str,
        commit_message: &str,
    ) -> miette::Result<String> {
        operations::merge_branches(self, base, head, commit_message).await
    }

    async fn post_comment(&self, pr: u64, text: &str) -> miette::Result<Comment> {
        self.client
            .issues(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .create_comment(pr, text)
            .await
            .into_diagnostic()
    }

    async fn get_revision(&self, branch: &str) -> miette::Result<String> {
        let reference = self
            .client
            .repos(
                self.repository_name.owner(),
                self.repository_name.repository(),
            )
            .get_ref(&Reference::Branch(branch.to_string()))
            .await
            .into_diagnostic()?;

        let Object::Commit { sha, .. } = reference.object else {
            return Err(Report::msg("invalid reference"));
        };

        Ok(sha)
    }

    async fn set_branch_to_revision(&self, branch: &str, revision: &str) -> miette::Result<()> {
        operations::set_branch_to_revision(self, branch.to_string(), revision.to_string()).await
    }
}

type RepositoryMap = HashMap<GithubRepositoryName, RepositoryState>;
type RepositoryState = GithubRepositoryState<GithubRepositoryClient>;
