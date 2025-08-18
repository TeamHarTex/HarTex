/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # Error (and Panic) Handler
//!
//! This module defines handlers for errors and panics.

use std::{env, str::FromStr};

use chrono::Utc;
use hartex_discord_core::discord::{
    http::client::InteractionClient,
    model::{
        gateway::payload::incoming::InteractionCreate,
        id::{Id, marker::ChannelMarker},
    },
    util::builder::embed::{EmbedBuilder, EmbedFieldBuilder},
};
use hartex_discord_utils::{
    CLIENT, interaction::ephemeral_error_response, markdown::MarkdownStyle,
};
use hartex_localization_core::Localizer;
use miette::Report;
use sha2::{Digest, Sha224};

/// This function handle errors from an interaction. It does the following things:
///
/// (1) generate a unique error code;
/// (2) send a message to a designated channel for error logs in the support server with the error code; and
/// (3) responds to the interaction with the error message with the error code.
pub async fn handle_interaction_error(
    payload: ErrorPayload,
    interaction_create: Box<InteractionCreate>,
    interaction_client: &InteractionClient<'_>,
    localizer: &Localizer<'_>,
) {
    let mut hasher = Sha224::new();

    let channel_id_str = env::var("ERROR_CHANNEL_ID").unwrap();
    let channel_id = Id::<ChannelMarker>::from_str(channel_id_str.as_str()).unwrap();

    let error_line_two = localizer.error_error_line_two().unwrap();

    match payload {
        ErrorPayload::Miette(report) => {
            let report = strip_ansi_escapes::strip_str(report.to_string());

            hasher.update(report.as_bytes());
            hasher.update(Utc::now().timestamp().to_string().as_bytes());

            let error_line_one = localizer.error_error_line_one("unexpected").unwrap();

            let output = hasher.finalize();
            let hash = output.map(|int| format!("{int:x}")).join("");
            interaction_client
                .create_response(
                    interaction_create.id,
                    &interaction_create.token,
                    &ephemeral_error_response(hartex_tracing::format!(
                        "{error_line_one}\n\n{error_line_two} {hash.clone().inline_code()}"
                    )),
                )
                .await
                .unwrap();

            let embed = EmbedBuilder::new()
                .color(0xFF_99_33)
                .title("Unexpected Error")
                .field(EmbedFieldBuilder::new(
                    "Error Hash",
                    hash.clone().inline_code(),
                ))
                .field(EmbedFieldBuilder::new("Error", report.clone().codeblock()))
                .validate()
                .unwrap()
                .build();

            CLIENT
                .create_message(channel_id)
                .embeds(&[embed])
                .await
                .unwrap();

            hartex_tracing::warn!("command errorred: {report:?}; error hash: {hash}");
        }
        ErrorPayload::Panic(message) => {
            let message = strip_ansi_escapes::strip_str(message);

            hasher.update(message.as_bytes());
            hasher.update(Utc::now().timestamp().to_string().as_bytes());

            let error_line_one = localizer.error_error_line_one("critical").unwrap();

            let output = hasher.finalize();
            let hash = output.map(|int| format!("{int:x}")).join("");

            interaction_client
                .create_response(
                    interaction_create.id,
                    &interaction_create.token,
                    &ephemeral_error_response(hartex_tracing::format!(
                        "{error_line_one}\n\n{error_line_two} {hash.clone().inline_code()}"
                    )),
                )
                .await
                .unwrap();

            let embed = EmbedBuilder::new()
                .color(0xFF_33_33)
                .title("Critical Error")
                .field(EmbedFieldBuilder::new(
                    "Error Hash",
                    hash.clone().inline_code(),
                ))
                .field(EmbedFieldBuilder::new("Error", message.clone().codeblock()))
                .validate()
                .unwrap()
                .build();

            CLIENT
                .create_message(channel_id)
                .embeds(&[embed])
                .await
                .unwrap();

            hartex_tracing::error!("interaction command panicked: {message:?}; error hash: {hash}");
        }
    }
}

/// The error payload received.
pub enum ErrorPayload {
    /// A `miette` report payload.
    Miette(Report),
    /// A panic message payload.
    Panic(String),
}
