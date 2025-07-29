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

//! # The About Command
//!
//! This command returns brief information about the bot, like its description and GitHub
//! repository.

use async_trait::async_trait;
use hartex_discord_commands_core::{command, context::CommandContext, traits::Command};
use hartex_discord_core::discord::{
    model::channel::message::component::UnfurledMediaItem,
    util::builder::message::{
        ContainerBuilder, SectionBuilder, SeparatorBuilder, TextDisplayBuilder, ThumbnailBuilder,
    },
};
use hartex_discord_utils::interaction::component_response;

use crate::general::General;

/// The `about` command declaration.
#[command(name = "about", plugin = General)]
pub struct About;

#[async_trait]
impl Command for About {
    async fn execute(&self, context: &CommandContext<'_>) -> miette::Result<()> {
        let about_embed_title = context.localizer.general_plugin_about_embed_title()?;
        let about_embed_description = context.localizer.general_plugin_about_embed_description()?;
        let about_embed_github_repo_field_name = context
            .localizer
            .general_plugin_about_embed_github_repo_field_name()?;
        let about_embed_footer = context
            .localizer
            .general_plugin_about_embed_footer("https://discord.gg/Xu8453VBAv")?;

        let thumbnail = ThumbnailBuilder::new(UnfurledMediaItem {
            url: "https://cdn.discordapp.com/avatars/936431574310879332/9a46b39c031ca84e8351ee97867afc96.png".into(),
            proxy_url: None,
            height: None,
            width: None,
            content_type: None,
        }).build();

        let title = TextDisplayBuilder::new(format!("# {about_embed_title}")).build();
        let description = TextDisplayBuilder::new(about_embed_description).build();
        let github_repo = TextDisplayBuilder::new(format!(
            "{about_embed_github_repo_field_name}: https://github.com/TeamHarTex/HarTex"
        ))
        .build();

        let footer = TextDisplayBuilder::new(format!("-# {about_embed_footer}")).build();

        let section = SectionBuilder::new(thumbnail)
            .component(title)
            .build();

        let container = ContainerBuilder::new()
            .accent_color(Some(0x41_A0_DE))
            .component(section)
            .component(SeparatorBuilder::new().build())
            .component(description)
            .component(github_repo)
            .component(SeparatorBuilder::new().build())
            .component(footer)
            .id(1)
            .spoiler(false)
            .build();

        // let embed = EmbedBuilder::new()
        //     .author(
        //         EmbedAuthorBuilder::new(about_embed_title)
        //             .icon_url(ImageSource::url("https://cdn.discordapp.com/avatars/936431574310879332/9a46b39c031ca84e8351ee97867afc96.png").into_diagnostic()?)
        //             .build()
        //     )
        //     .color(0x41_A0_DE)
        //     .description(about_embed_description)
        //     .field(EmbedFieldBuilder::new(about_embed_github_repo_field_name, "https://github.com/TeamHarTex/HarTex").build())
        //     .footer(EmbedFooterBuilder::new(about_embed_footer).build())
        //     .validate()
        //     .into_diagnostic()?
        //     .build();

        context
            .create_response(component_response(vec![container]))
            .await?;

        Ok(())
    }
}
