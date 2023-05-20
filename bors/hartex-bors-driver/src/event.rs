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

use hartex_bors_github::GithubBorsState;
use hartex_eyre::eyre::Report;
use octocrab::models::events::payload::IssueCommentEventPayload;
use serde_json::Value;

/// Bors event
pub enum BorsEvent {
    IssueComment(IssueCommentEventPayload),
}

/// Deserialize an event.
pub fn deserialize_event(event_type: String, event_json: Value) -> hartex_eyre::Result<BorsEvent> {
    match &*event_type {
        "issue_comment" => {
            let deserialized = serde_json::from_value::<IssueCommentEventPayload>(event_json)?;
            if deserialized.issue.pull_request.is_none() {
                return Err(Report::msg("comments on non-pull requests are ignored"));
            }

            Ok(BorsEvent::IssueComment(deserialized))
        }
        _ => Err(Report::msg("unsupported events are ignored")),
    }
}

/// Handke an event.
#[allow(dead_code)]
pub async fn handle_event(_: &mut GithubBorsState, _: BorsEvent) -> hartex_eyre::Result<()> {
    todo!()
}
