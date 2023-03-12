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

use std::env;

use hartex_discord_core::discord::model::application::interaction::InteractionType;
use hartex_discord_core::discord::model::gateway::event::DispatchEvent;
use hartex_discord_core::discord::model::gateway::event::GatewayEvent;
use hartex_discord_core::log;
use hartex_discord_core::scylla::SessionBuilder;
use hartex_discord_core::scylla::transport::Compression;

pub async fn invoke(event: GatewayEvent, shard: u8) -> hartex_eyre::Result<()> {
    #[allow(clippy::collapsible_match)]
    match event {
        GatewayEvent::Dispatch(seq, dispatch) => match dispatch {
            DispatchEvent::InteractionCreate(interaction_create)
                if interaction_create.kind == InteractionType::ApplicationCommand =>
            {
                crate::interaction::application_command(interaction_create).await
            }
            DispatchEvent::Ready(ready) => {
                log::info!(
                    "{}#{} (shard {shard}) has received READY payload from Discord (gateway v{}) (sequence {seq})",
                    ready.user.name,
                    ready.user.discriminator,
                    ready.version
                );

                let username = env::var("HARTEX_NIGHTLY_SCYLLADB_USERNAME")?;
                let passwd = env::var("HARTEX_NIGHTLY_SCYLLADB_PASSWORD")?;
                let _ = SessionBuilder::new()
                    .known_node("localhost:9042")
                    .compression(Some(Compression::Lz4))
                    .user(username, passwd)
                    .build()
                    .await?;

                Ok(())
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}
