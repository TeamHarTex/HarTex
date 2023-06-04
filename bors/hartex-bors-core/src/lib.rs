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

//! # Bors Core Library

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(async_fn_in_trait)]

use std::future::Future;
use std::pin::Pin;

use octocrab::models::issues::Comment;
use octocrab::models::pulls::PullRequest;
use octocrab::models::CommentId;
use octocrab::models::RunId;

use crate::models::BorsBuild;
use crate::models::BorsBuildStatus;
use crate::models::BorsPullRequest;
use crate::models::BorsRepository;
use crate::models::BorsWorkflow;
use crate::models::BorsWorkflowStatus;
use crate::models::BorsWorkflowType;
use crate::models::Check;
use crate::models::GithubRepositoryName;
use crate::models::GithubRepositoryState;
use crate::models::Permission;

pub mod models;

/// A state of bors.
pub trait BorsState<C: RepositoryClient> {
    /// Checks whether the comment is posted by bors itself.
    fn comment_posted_by_bors(&self, comment: Comment) -> bool;

    /// Returns a mutable reference to the repository state by its name.
    fn get_repository_state_mut(
        &mut self,
        repository: &GithubRepositoryName,
    ) -> Option<(&mut GithubRepositoryState<C>, &mut dyn DatabaseClient)>;
}

/// A database client.
pub trait DatabaseClient {
    /// Associate a try build to a pull request.
    fn associate_try_build<'a>(
        &'a self,
        pr: &'a BorsPullRequest,
        branch: String,
        commit_hash: String,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>>;

    /// Creates a bors repository.
    ///
    /// If the repository already exists, this function does nothing.
    fn create_repository<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>>;

    /// Creates a workflow.
    fn create_workflow<'a>(
        &'a self,
        build: &'a BorsBuild,
        name: String,
        url: String,
        run_id: RunId,
        workflow_type: BorsWorkflowType,
        workflow_status: BorsWorkflowStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>>;

    /// Finds a build.
    fn find_build<'a>(
        &'a self,
        repo: &'a GithubRepositoryName,
        branch: String,
        commit_sha: String,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Option<BorsBuild>>> + '_>>;

    /// Find a pull request from a build.
    fn find_pull_request_by_build<'a>(
        &'a self,
        build: &'a BorsBuild,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Option<BorsPullRequest>>> + '_>>;

    /// Gets a bors pull request in the bors database, or creates before returning if the pull
    /// request is not present yet.
    fn get_or_create_pull_request<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
        pr: u64,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<BorsPullRequest>> + '_>>;

    /// Gets pull requests stored in the database in a repository.
    fn get_pull_requests_for_repository<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsPullRequest>>> + Send + '_>>;

    /// Gets the repositories bors is installed in.
    fn get_repositories(
        &self,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsRepository>>> + Send + '_>>;

    /// Gets the workflows for a certain build.
    fn get_workflows_for_build<'a>(
        &'a mut self,
        build: &'a BorsBuild,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<Vec<BorsWorkflow>>> + '_>>;

    /// Update the status of a build.
    fn update_build_status<'a>(
        &'a self,
        build: &'a BorsBuild,
        status: BorsBuildStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>>;

    /// Update the status of a workflow.
    fn update_workflow_status(
        &self,
        run_id: u64,
        status: BorsWorkflowStatus,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<()>> + '_>>;
}

/// A base permission resolver.
pub trait PermissionResolver {
    /// Resolves permissions for a user and returns whether that user has the specified permission.
    fn resolve_user<'a>(
        &'a self,
        username: &'a str,
        permission: Permission,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;
}

/// A repository client.
pub trait RepositoryClient {
    /// The name of the repository this client is for.
    fn repository_name(&self) -> &GithubRepositoryName;

    /// Cancel workflows.
    async fn cancel_workflows(&mut self, run_ids: Vec<RunId>) -> hartex_eyre::Result<()>;

    /// Edit a speific comment.
    async fn edit_comment(
        &mut self,
        comment_id: CommentId,
        text: &str,
    ) -> hartex_eyre::Result<Comment>;

    async fn get_checks_for_commit(
        &mut self,
        branch: &str,
        commit_hash: &str,
    ) -> hartex_eyre::Result<Vec<Check>>;

    /// Gets a pull request by its number.
    async fn get_pull_request(&mut self, pr: u64) -> hartex_eyre::Result<PullRequest>;

    /// Merges two branches together.
    async fn merge_branches(
        &mut self,
        base: &str,
        head: &str,
        commit_message: &str,
    ) -> hartex_eyre::Result<String>;

    /// Post a comment on a specific pull request.
    async fn post_comment(&mut self, pr: u64, text: &str) -> hartex_eyre::Result<Comment>;

    /// Set a branch to a specific revision.
    async fn set_branch_to_revision(
        &mut self,
        branch: &str,
        revision: &str,
    ) -> hartex_eyre::Result<()>;
}
