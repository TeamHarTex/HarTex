/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use base::discord::model::application::command::CommandType;
use base::discord::model::application::interaction::{Interaction, InteractionData};
use base::discord::model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use base::error::Result;
use hyper::Client;

use crate::{BaseInteraction, HandleInteractionFuture};

pub struct PingCommand;

impl BaseInteraction for PingCommand {
    fn handle(&self, interaction: Interaction, loadbal_port: u16) -> HandleInteractionFuture {
        let Some(InteractionData::ApplicationCommand(data)) = interaction.clone().data else {
            unreachable!()
        };

        if data.kind != CommandType::ChatInput {
            unreachable!()
        }

        Box::pin(ping_chat_input(interaction, loadbal_port))
    }
}

async fn ping_chat_input(interaction: Interaction, loadbal_port: u16) -> Result<()> {
    log::trace!("command `ping` was executed");

    tokio::spawn(Client::new().request(
        restreq::create_interaction_response::create_interaction_response(
            interaction.id.get(),
            interaction.token.clone(),
            InteractionResponse {
                data: Some(InteractionResponseData {
                    allowed_mentions: None,
                    attachments: None,
                    choices: None,
                    components: None,
                    content: Some(String::from("Pong! Did you need anything? :eyes:")),
                    custom_id: None,
                    embeds: None,
                    flags: None,
                    title: None,
                    tts: None,
                }),
                kind: InteractionResponseType::ChannelMessageWithSource,
            },
            loadbal_port,
        )?,
    ));

    Ok(())
}
