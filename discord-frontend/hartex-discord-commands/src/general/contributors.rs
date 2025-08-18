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

//! # The Contributors Command
//!
//! This command returns information about the contributors of the bot.

use async_trait::async_trait;
use hartex_discord_commands_core::{command, context::CommandContext, traits::Command};
use hartex_discord_core::discord::{
    model::channel::message::component::SeparatorSpacingSize,
    util::builder::message::{ContainerBuilder, SeparatorBuilder, TextDisplayBuilder},
};
use hartex_discord_utils::{interaction::component_response, markdown::MarkdownStyle};

use crate::general::General;

/// The `contributors` command declaration.
#[command(name = "contributors", plugin = General)]
pub struct Contributors;

#[async_trait]
impl Command for Contributors {
    async fn execute(&self, context: &CommandContext<'_>) -> miette::Result<()> {
        let contributors_embed_title = context
            .localizer
            .general_plugin_contributors_embed_title()?;
        let contributors_embed_description = context
            .localizer
            .general_plugin_contributors_embed_description()?;
        let contributors_embed_global_admin_field_name = context
            .localizer
            .general_plugin_contributors_embed_global_admin_field_name()?;
        let contributors_embed_front_dev_field_name = context
            .localizer
            .general_plugin_contributors_embed_front_dev_field_name()?;
        let contributors_embed_translation_team_field_name = context
            .localizer
            .general_plugin_contributors_embed_translation_team_field_name()?;
        let contributors_embed_footer = context
            .localizer
            .general_plugin_contributors_embed_footer()?;

        let title_desc = TextDisplayBuilder::new(formati::format!(
            "{contributors_embed_title.h1()}\n{contributors_embed_description}"
        ))
        .build();

        let glob_admin = TextDisplayBuilder::new(format!(
            "{contributors_embed_global_admin_field_name.h2()}\nhtgazurex1212."
        ))
        .build();

        let front = TextDisplayBuilder::new(format!(
            "{contributors_embed_front_dev_field_name.h2()}\narizlunari"
        ))
        .build();

        let translate = TextDisplayBuilder::new(format!(
            "{contributors_embed_translation_team_field_name.h1()}\nmadonuko (`ja`)\nteddyji (`zh-CN`)\nxzihnago (`zh-TW`)"
        ))
        .build();

        let footer = TextDisplayBuilder::new(contributors_embed_footer.footnote()).build();

        let component = ContainerBuilder::new()
            .accent_color(Some(0x41_A0_DE))
            .component(title_desc)
            .component(
                SeparatorBuilder::new()
                    .spacing(SeparatorSpacingSize::Small)
                    .build(),
            )
            .component(glob_admin)
            .component(front)
            .component(translate)
            .component(
                SeparatorBuilder::new()
                    .spacing(SeparatorSpacingSize::Small)
                    .build(),
            )
            .component(footer)
            .build();

        context
            .create_response(component_response(vec![component]))
            .await?;

        Ok(())
    }
}
