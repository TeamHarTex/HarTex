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
use hartex_bors_core::BorsState;
use hartex_bors_core::DatabaseClient;
use hartex_bors_core::RepositoryClient;
use hartex_bors_github::webhook::WebhookRepository;
use hartex_bors_github::GithubBorsState;
use hartex_eyre::eyre::Report;
use hartex_log::log;
use octocrab::models::events::payload::IssueCommentEventAction;
use octocrab::models::events::payload::IssueCommentEventPayload;
use octocrab::models::events::payload::WorkflowRunEventAction;
use octocrab::models::events::payload::WorkflowRunEventPayload;
use octocrab::models::issues::Comment;
use octocrab::models::issues::Issue;
use serde_json::Value;

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
    /// A workflow run.
    WorkflowRun(WorkflowRunEventPayload),
}

/// Deserialize an event.
pub fn deserialize_event(event_type: String, event_json: Value) -> hartex_eyre::Result<BorsEvent> {
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

            let repository = serde_json::from_value::<WebhookRepository>(event_json)?;

            Ok(BorsEvent {
                kind: BorsEventKind::IssueComment(deserialized),
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
    state: &mut GithubBorsState,
    event: BorsEvent,
) -> hartex_eyre::Result<()> {
    match event.kind {
        BorsEventKind::IssueComment(payload) => {
            if state.comment_posted_by_bors(payload.comment.clone()) {
                log::trace!("ignoring comment posted by myself");
                return Ok(());
            }

            if let Some((repository, database)) = retrieve_repository_state(
                state,
                &GithubRepositoryName::new_from_repository(event.repository.repository)?,
            ) {
                if let Err(error) =
                    handle_comment(repository, database, payload.comment, payload.issue).await
                {
                    println!("{error}");
                }
            }
        }
        BorsEventKind::WorkflowRun(payload)
            if payload.action == WorkflowRunEventAction::InProgress =>
        {
            if let Some((repository, database)) = retrieve_repository_state(
                state,
                &GithubRepositoryName::new_from_repository(event.repository.repository)?,
            ) {
                crate::workflows::workflow_started(repository, database, payload.workflow_run)
            }
        }
        BorsEventKind::WorkflowRun(payload)
            if payload.action == WorkflowRunEventAction::Completed => {
            if let Some((repository, database)) = retrieve_repository_state(
                state,
                &GithubRepositoryName::new_from_repository(event.repository.repository)?,
            ) {
                crate::workflows::workflow_completed(repository, database, payload.workflow_run)
            }
        }
        _ => return Err(Report::msg("unsupported event payloads are ignored")),
    }

    Ok(())
}

async fn handle_comment<C: RepositoryClient>(
    repository: &mut GithubRepositoryState<C>,
    database: &mut dyn DatabaseClient,
    comment: Comment,
    issue: Issue,
) -> hartex_eyre::Result<()> {
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
                BorsCommand::Ping => {
                    hartex_bors_commands::commands::ping::ping_command(repository, pr).await?
                }
                BorsCommand::Try => {
                    hartex_bors_commands::commands::r#try::try_command(
                        repository,
                        database,
                        pr,
                        &issue.user.login,
                    )
                    .await?
                }
                _ => todo!(),
            },
            Err(error) => {
                let error_msg = match error {
                    ParserError::MissingCommand => "Missing command.".to_string(),
                    ParserError::UnknownCommand(command) => {
                        format!(r#"Unknown command "{command}"."#)
                    }
                };

                repository.client.post_comment(pr, &error_msg).await?;
            }
        }
    }

    Ok(())
}

fn retrieve_repository_state<'a, C: RepositoryClient>(
    state: &'a mut dyn BorsState<C>,
    repository: &GithubRepositoryName,
) -> Option<(&'a mut GithubRepositoryState<C>, &'a mut dyn DatabaseClient)> {
    match state.get_repository_state_mut(repository) {
        Some(result) => Some(result),
        None => {
            log::warn!("repository {repository} not found");
            None
        }
    }
}
