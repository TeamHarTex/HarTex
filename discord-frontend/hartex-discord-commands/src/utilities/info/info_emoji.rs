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

use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandOptionValue;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;

#[allow(clippy::unused_async)]
pub async fn execute(interaction: Interaction, option: CommandDataOption) -> miette::Result<()> {
    let CommandOptionValue::SubCommand(options) = option.value else {
        unreachable!()
    };

    let _ = CLIENT.interaction(interaction.application_id);
    let locale = interaction.locale.unwrap_or_else(|| String::from("en-GB"));
    let _ = Localizer::new(&LOCALIZATION_HOLDER, &locale);

    let CommandOptionValue::String(_) = options
        .iter()
        .find(|option| option.name.as_str() == "emoji")
        .map_or(CommandOptionValue::String(String::new()), |option| {
            option.value.clone()
        })
    else {
        unreachable!()
    };

    Ok(())
}
