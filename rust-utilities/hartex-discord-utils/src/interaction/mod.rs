/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

//! # Utilities for Interaction Handling

use hartex_discord_core::discord::{
    model::{
        channel::message::{Component, Embed, MessageFlags},
        http::interaction::{InteractionResponse, InteractionResponseType},
    },
    util::builder::InteractionResponseDataBuilder,
};

mod components;

/// Constructs a response with components
#[must_use]
pub fn component_response(
    components: impl IntoIterator<Item = impl Into<Component>>,
) -> InteractionResponse {
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .components(components.into_iter().map(Into::into))
                .flags(MessageFlags::IS_COMPONENTS_V2)
                .build(),
        ),
    }
}

/// Constructs an embed response.
#[must_use]
#[deprecated(since = "0.16.0", note = "embeds in responses are deprecated")]
pub fn embed_response(embeds: Vec<Embed>) -> InteractionResponse {
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseDataBuilder::new().embeds(embeds).build()),
    }
}

/// Constructs an ephemeral text response, used for errors.
#[must_use]
pub fn ephemeral_error_response(message: impl Into<String>) -> InteractionResponse {
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(message)
                .flags(MessageFlags::EPHEMERAL)
                .build(),
        ),
    }
}
