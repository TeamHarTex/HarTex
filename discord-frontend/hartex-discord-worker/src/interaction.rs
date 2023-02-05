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

use hartex_discord_commands::general::about::About;
use hartex_discord_commands::general::latency::Latency;
use hartex_discord_commands::general::uptime::Uptime;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::traits::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::InteractionData;
use hartex_discord_core::discord::model::gateway::payload::incoming::InteractionCreate;

pub async fn application_command(
    interaction_create: Box<InteractionCreate>,
) -> hartex_eyre::Result<()> {
    let InteractionData::ApplicationCommand(command) = interaction_create.data.clone().unwrap() else {
        unreachable!("this should not be possible")
    };

    match command.name {
        name if name == About.name() => About.execute(interaction_create.0).await,
        name if name == Latency.name() => Latency.execute(interaction_create.0).await,
        name if name == Uptime.name() => Uptime.execute(interaction_create.0).await,
        _ => Ok(()),
    }
}
