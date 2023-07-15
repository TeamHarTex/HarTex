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

//! # Bors Event Model

use hartex_bors_commands::parser::ParserError;
use hartex_bors_commands::BorsCommand;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::models::GithubRepositoryState;
use hartex_bors_core::queue::BorsQueueEvent;
use hartex_bors_core::BorsState;
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use hartex_bors_github::webhook::WebhookRepository;
use hartex_bors_github::GithubBorsState;
use hartex_log::log;
use miette::Report;
use octocrab::models::events::payload::IssueCommentEventAction;
use octocrab::models::events::payload::IssueCommentEventPayload;
use octocrab::models::events::payload::PullRequestEventAction;
use octocrab::models::events::payload::PullRequestEventPayload;
use octocrab::models::events::payload::WorkflowRunEventAction;
use octocrab::models::events::payload::WorkflowRunEventPayload;
use octocrab::models::issues::Comment;
use octocrab::models::issues::Issue;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

/// Bors event
pub struct BorsEvent {
    /// The kind of event.
    pub kind: BorsEventKind,
    /// The repository the event is sent from.
    pub repository: WebhookRepository,
}

/// The kind of event.
pub enum BorsEventKind {
    /// An issue comment.
    IssueComment(IssueCommentEventPayload),
    /// An issue comment.
    PullRequest(PullRequestEventPayload),
    /// A workflow run.
    WorkflowRun(WorkflowRunEventPayload),
}

/// Deserialize an event.
pub fn deserialize_event(event_type: String, event_json: Value) -> miette::Result<BorsEvent> {
    match &*event_type {
        "issue_comment" => {
            let deserialized =
                serde_json::from_value::<IssueCommentEventPayload>(event_json.clone())?;

            if deserialized.action != IssueCommentEventAction::Created {
                return Err(Report::msg("non created issue comments are ignored"));
            }

            if deserialized.issue.pull_request.is_none() {
                return Err(Report::msg("comments on non-pull requests are ignored"));
            }

            if deserialized.issue.closed_at.is_some() {
                return Err(Report::msg("comments on closed pull requests are ignored"));
            }

            let repository = serde_json::from_value::<WebhookRepository>(event_json)?;

            Ok(BorsEvent {
                kind: BorsEventKind::IssueComment(deserialized),
                repository,
            })
        }
        "pull_request" => {
            let deserialized =
                serde_json::from_value::<PullRequestEventPayload>(event_json.clone())?;

            if deserialized.action != PullRequestEventAction::Opened {
                return Err(Report::msg("non opened pull requests are ignored"));
            }

            let repository = serde_json::from_value::<WebhookRepository>(event_json)?;

            Ok(BorsEvent {
                kind: BorsEventKind::PullRequest(deserialized),
                repository,
            })
        }
        "workflow_run" => {
            let deserialized =
                serde_json::from_value::<WorkflowRunEventPayload>(event_json.clone())?;
            let repository = serde_json::from_value::<WebhookRepository>(event_json)?;

            Ok(BorsEvent {
                kind: BorsEventKind::WorkflowRun(deserialized),
                repository,
            })
        }
        _ => Err(Report::msg("unsupported events are ignored")),
    }
}

/// Handke an event.
pub async fn handle_event(
    state: &GithubBorsState,
    event: BorsEvent,
) -> miette::Result<()> {
    match event.kind {
        BorsEventKind::IssueComment(payload) => {
            if state.comment_posted_by_bors(payload.comment.clone()) {
                log::trace!("ignoring comment posted by myself");
                return Ok(());
            }

            if let Some((repository, database, sender)) = retrieve_repository_state(
                state,
                &GithubRepositoryName::new_from_repository(event.repository.repository)?,
            ) {
                if let Err(error) =
                    handle_comment(repository, database, payload.comment, payload.issue, sender)
                        .await
                {
                    println!("{error}");
                }
            }
        }
        BorsEventKind::PullRequest(payload) => {
            let repository_name =
                GithubRepositoryName::new_from_repository(event.repository.repository)?;

            if let Some((repository, database, _)) =
                retrieve_repository_state(state, &repository_name)
            {
                repository
                    .client
                    .client()
                    .issues(repository_name.owner(), repository_name.repository())
                    .add_labels(payload.number, &[String::from("waiting-on-review")])
                    .await?;

                database
                    .get_or_create_pull_request(
                        &repository_name,
                        &payload.pull_request,
                        payload.number,
                    )
                    .await?;
            }
        }
        BorsEventKind::WorkflowRun(payload)
            if payload.action == WorkflowRunEventAction::InProgress =>
        {
            if let Some((repository, database, _)) = retrieve_repository_state(
                state,
                &GithubRepositoryName::new_from_repository(event.repository.repository)?,
            ) {
                crate::workflows::workflow_started(repository, database, payload.workflow_run)
                    .await?;
            }
        }
        BorsEventKind::WorkflowRun(payload)
            if payload.action == WorkflowRunEventAction::Completed =>
        {
            if let Some((repository, database, sender)) = retrieve_repository_state(
                state,
                &GithubRepositoryName::new_from_repository(event.repository.repository)?,
            ) {
                crate::workflows::workflow_completed(repository, database, payload.workflow_run, sender)
                    .await?;
            }
        }
        _ => return Err(Report::msg("unsupported event payloads are ignored")),
    }

    Ok(())
}

async fn handle_comment<C: RepositoryClient>(
    repository: &GithubRepositoryState<C>,
    database: &dyn DatabaseClient,
    comment: Comment,
    issue: Issue,
    sender: Sender<BorsQueueEvent>,
) -> miette::Result<()> {
    let pr = issue.number;
    let body = comment.body.unwrap();
    let commands = hartex_bors_commands::parse_commands(&body);

    log::info!(
        "received comment at https://github.com/{}/{}/issues/{}, commands: {:?}",
        repository.repository.owner(),
        repository.repository.repository(),
        pr,
        commands,
    );

    for command in commands {
        match command {
            Ok(command) => match command {
                BorsCommand::Approve => {
                    hartex_bors_commands::commands::approve::approve_command(
                        repository,
                        database,
                        pr,
                        &comment.user.login,
                        sender.clone(),
                    )
                    .await?
                }
                BorsCommand::ApproveEq { reviewer } => {
                    hartex_bors_commands::commands::approve::approve_command(
                        repository, database, pr, &reviewer, sender.clone(),
                    )
                    .await?
                }
                BorsCommand::Ping => {
                    hartex_bors_commands::commands::ping::ping_command(repository, pr).await?
                }
                BorsCommand::Try { parent } => {
                    hartex_bors_commands::commands::r#try::try_command(
                        repository,
                        database,
                        pr,
                        &comment.user.login,
                        parent,
                    )
                    .await?
                }
                BorsCommand::TryCancel => {
                    hartex_bors_commands::commands::try_cancel::try_cancel_command(
                        repository,
                        database,
                        pr,
                        &comment.user.login,
                    )
                    .await?
                }
            },
            Err(error) => {
                let error_msg = match error {
                    ParserError::MissingCommand => "Missing command.".to_string(),
                    ParserError::UnexpectedEndOfCommand => "Unexpected end of command.".to_string(),
                    ParserError::UnknownCommand(command) => {
                        format!(r#"Unknown command "{command}"."#)
                    }
                    _ => format!("An error occurred."),
                };

                repository.client.post_comment(pr, &error_msg).await?;
            }
        }
    }

    Ok(())
}

fn retrieve_repository_state<'a, C: RepositoryClient>(
    state: &'a dyn BorsState<C>,
    repository: &GithubRepositoryName,
) -> Option<(
    &'a GithubRepositoryState<C>,
    &'a dyn DatabaseClient,
    Sender<BorsQueueEvent>,
)> {
    match state.get_repository_state(repository) {
        Some(result) => Some(result),
        None => {
            log::warn!("repository {repository} not found");
            None
        }
    }
}
