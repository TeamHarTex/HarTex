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
use base::discord::util::builder::embed::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder, ImageSource,
};
use base::error::Result;
use hyper::Client;

use crate::{BaseInteraction, HandleInteractionFuture};

pub struct AboutCommand;

impl BaseInteraction for AboutCommand {
    fn handle(&self, interaction: Interaction, loadbal_port: u16) -> HandleInteractionFuture {
        let Some(InteractionData::ApplicationCommand(ref data)) = interaction.data else {
            unreachable!()
        };

        if data.kind != CommandType::ChatInput {
            unreachable!()
        }

        Box::pin(about_chat_input(interaction, loadbal_port))
    }
}

async fn about_chat_input(interaction: Interaction, loadbal_port: u16) -> Result<()> {
    log::trace!("command `about` was executed");

    #[cfg(stable)]
    let embed = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new("HarTex").icon_url(ImageSource::url("https://cdn.discordapp.com/avatars/936431574310879332/9a46b39c031ca84e8351ee97867afc96.png?size=256").unwrap()));
    #[cfg(not(stable))]
    let embed = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new("HarTex Nightly").icon_url(ImageSource::url("https://cdn.discordapp.com/avatars/936432439767740436/388c658aedb5f9ec5f99b85159514451.png?size=256").unwrap()));

    let embed = embed.description("HarTex is an advanced administration assistant and moderation bot for Discord, designed with stability and flexibility in mind.")
        .footer(EmbedFooterBuilder::new("Powered by Open Source, Powered by a Community: https://github.com/TeamHarTex"))
        .validate()
        .unwrap()
        .build();

    tokio::spawn(Client::new().request(
        restreq::create_interaction_response::create_interaction_response(
            interaction.id.get(),
            interaction.token,
            InteractionResponse {
                data: Some(InteractionResponseData {
                    embeds: Some(vec![embed]),
                    ..Default::default()
                }),
                kind: InteractionResponseType::ChannelMessageWithSource,
            },
            loadbal_port,
        )?,
    ));

    Ok(())
}
