/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

use std::env;
use std::str::FromStr;

use chrono::Utc;
use hartex_discord_core::discord::model::gateway::payload::incoming::InteractionCreate;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::model::id::marker::ChannelMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_log::log;
use miette::Report;
use sha2::Digest;
use sha2::Sha224;

pub async fn handle_interaction_error(
    payload: ErrorPayload,
    interaction_create: Box<InteractionCreate>,
) {
    let mut hasher = Sha224::new();

    let channel_id_str = env::var("ERROR_CHANNEL_ID").unwrap();
    let channel_id = Id::<ChannelMarker>::from_str(channel_id_str.as_str()).unwrap();

    match payload {
        ErrorPayload::Miette(report) => {
            let report = strip_ansi_escapes::strip_str(report.to_string());

            hasher.update(report.as_bytes());
            hasher.update(Utc::now().timestamp().to_string().as_bytes());

            let output = hasher.finalize();
            let hash = output.map(|int| format!("{int:x}")).join("");

            let interaction_client = CLIENT.interaction(interaction_create.application_id);
            interaction_client
                .create_response(
                    interaction_create.id,
                    &interaction_create.token,
                    &InteractionResponse {
                        kind: InteractionResponseType::ChannelMessageWithSource,
                        data: Some(
                            InteractionResponseDataBuilder::new()
                                .content(format!(
                                    ":x: This command encountered an unexpected error. Please provide the following error code for support.\n\nError code: {}", hash.clone().discord_inline_code()
                                ))
                                .build(),
                        ),
                    },
                )
                .await
                .unwrap();

            let embed = EmbedBuilder::new()
                .color(0xFF9933)
                .title("Unexpected Error")
                .field(EmbedFieldBuilder::new(
                    "Error Hash",
                    hash.clone().discord_inline_code(),
                ))
                .field(EmbedFieldBuilder::new(
                    "Error",
                    report.clone().discord_codeblock(),
                ))
                .validate()
                .unwrap()
                .build();

            CLIENT
                .create_message(channel_id)
                .embeds(&[embed])
                .await
                .unwrap();

            log::warn!("command errorred: {report:?}; error hash: {hash}");
        }
        ErrorPayload::Panic(message) => {
            let message = strip_ansi_escapes::strip_str(message);

            hasher.update(message.as_bytes());
            hasher.update(Utc::now().timestamp().to_string().as_bytes());

            let output = hasher.finalize();
            let hash = output.map(|int| format!("{int:x}")).join("");

            let interaction_client = CLIENT.interaction(interaction_create.application_id);
            interaction_client
                .create_response(
                    interaction_create.id,
                    &interaction_create.token,
                    &InteractionResponse {
                        kind: InteractionResponseType::ChannelMessageWithSource,
                        data: Some(
                            InteractionResponseDataBuilder::new()
                                .content(format!(
                                    ":x: This command encountered a critical error. Please provide the following error code for support.\n\nError code: {}", hash.clone().discord_inline_code()
                                ))
                                .build(),
                        ),
                    },
                )
                .await
                .unwrap();

            let embed = EmbedBuilder::new()
                .color(0xFF3333)
                .title("Critical Error")
                .field(EmbedFieldBuilder::new(
                    "Error Hash",
                    hash.clone().discord_inline_code(),
                ))
                .field(EmbedFieldBuilder::new(
                    "Error",
                    message.clone().discord_codeblock(),
                ))
                .validate()
                .unwrap()
                .build();

            CLIENT
                .create_message(channel_id)
                .embeds(&[embed])
                .await
                .unwrap();

            log::error!("interaction command panicked: {message:?}; error hash: {hash}");
        }
    }
}

pub enum ErrorPayload {
    Miette(Report),
    Panic(String),
}
