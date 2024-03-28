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

use hartex_discord_core::discord::model::gateway::payload::incoming::InteractionCreate;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::CLIENT;
use hartex_log::log;
use miette::Report;

pub async fn handle_interaction_error(
    payload: ErrorPayload,
    interaction_create: Box<InteractionCreate>,
) {
    match payload {
        ErrorPayload::Miette(report) => {
            let interaction_client = CLIENT.interaction(interaction_create.application_id);
            interaction_client
                .create_response(
                    interaction_create.id,
                    &interaction_create.token,
                    &InteractionResponse {
                        kind: InteractionResponseType::ChannelMessageWithSource,
                        data: Some(
                            InteractionResponseDataBuilder::new()
                                .content("This command encountered an unexpected error.")
                                .build(),
                        ),
                    },
                )
                .await
                .unwrap();

            log::warn!("command errorred: {report}; error hash: ");
        }
        ErrorPayload::Panic(message) => {
            let interaction_client = CLIENT.interaction(interaction_create.application_id);
            interaction_client
                .create_response(
                    interaction_create.id,
                    &interaction_create.token,
                    &InteractionResponse {
                        kind: InteractionResponseType::ChannelMessageWithSource,
                        data: Some(
                            InteractionResponseDataBuilder::new()
                                .content("This command encountered a critical error.")
                                .build(),
                        ),
                    },
                )
                .await
                .unwrap();

            log::error!("interaction command panicked: {message}; error hash: ");
        }
    }
}

pub enum ErrorPayload {
    Miette(Report),
    Panic(String),
}
